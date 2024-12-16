use bsky_sdk::{
    api::{self, types::string::AtIdentifier},
    BskyAgent,
};
use std::{collections::HashSet, str::FromStr};
use tokio;

async fn get_followers(
    agent: &BskyAgent,
    actor: AtIdentifier,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    loop {
        let res = agent
            .api
            .app
            .bsky
            .graph
            .get_followers(
                api::app::bsky::graph::get_followers::ParametersData {
                    actor: actor.clone(),
                    cursor,
                    limit: None,
                }
                .into(),
            )
            .await?;
        cursor = res.cursor.clone();
        for f in res.followers.iter() {
            found.insert(AtIdentifier::Did(f.did.clone()));
        }
        if cursor.is_none() {
            break;
        }
    }

    println!("got {} followers of {:?}", found.len(), actor);
    Ok(found)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = BskyAgent::builder().build().await?;
    agent.login("gnmerritt.net", "APP-PW").await?;

    get_followers(&agent, AtIdentifier::from_str("gnmerritt.net")?).await?;
    Ok(())
}
