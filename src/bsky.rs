use std::collections::HashSet;

use bsky_sdk::{
    api::{self, types::string::AtIdentifier},
    BskyAgent,
};

pub(crate) async fn get_followers(
    agent: &BskyAgent,
    actor: &AtIdentifier,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    found.insert(actor.clone());
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

pub(crate) async fn get_follows(
    agent: &BskyAgent,
    actor: &AtIdentifier,
) -> Result<HashSet<AtIdentifier>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    found.insert(actor.clone());
    loop {
        let res = agent
            .api
            .app
            .bsky
            .graph
            .get_follows(
                api::app::bsky::graph::get_follows::ParametersData {
                    actor: actor.clone(),
                    cursor,
                    limit: None,
                }
                .into(),
            )
            .await?;
        cursor = res.cursor.clone();
        for f in res.follows.iter() {
            found.insert(AtIdentifier::Did(f.did.clone()));
        }
        if cursor.is_none() {
            break;
        }
    }
    println!("got {} followed by {:?}", found.len(), actor);
    Ok(found)
}
