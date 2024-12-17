pub mod bsky;

use bsky_sdk::api::types::string::AtIdentifier;
use bsky_sdk::BskyAgent;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Source {
    Followers { followers_of: AtIdentifier },
    Follows { followed_by: AtIdentifier },
}

async fn resolve_sources(
    agent: &BskyAgent,
    sources: &Vec<Source>,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    let mut matches = HashSet::new();
    for s in sources {
        matches.extend(get_matching_accounts(s, agent).await?);
    }
    Ok(matches)
}

async fn get_matching_accounts(
    source: &Source,
    agent: &BskyAgent,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    match source {
        Source::Followers { followers_of } => bsky::get_followers(agent, followers_of).await,
        Source::Follows { followed_by } => bsky::get_follows(agent, followed_by).await,
    }
}

#[derive(Deserialize)]
pub struct Blocklist {
    pub name: String,
    pub includes: Vec<Source>,
    #[serde(default)]
    pub excludes: Vec<Source>,
}

pub async fn resolve_blocklist(
    agent: &BskyAgent,
    list: &Blocklist,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    let mut included = resolve_sources(agent, &list.includes).await?;
    let excluded = resolve_sources(agent, &list.excludes).await?;
    included.retain(|a| !excluded.contains(a));
    Ok(included)
}

#[derive(Deserialize)]
pub struct BskyLogin {
    pub handle: String,
    pub app_password: Option<String>,
}

#[derive(Deserialize)]
pub struct EarmuffsConfig {
    pub auth: BskyLogin,
    pub lists: Vec<Blocklist>,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<EarmuffsConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let c = serde_json::from_reader(reader)?;
    Ok(c)
}
