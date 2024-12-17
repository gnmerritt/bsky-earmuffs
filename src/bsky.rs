use std::collections::{HashMap, HashSet};

use bsky_sdk::{
    api::{
        self,
        app::bsky::graph::defs::{ListViewData, MODLIST},
        types::{
            string::{AtIdentifier, Datetime, Did},
            Collection, TryIntoUnknown,
        },
    },
    BskyAgent,
};

pub(crate) async fn get_followers(
    agent: &BskyAgent,
    actor: &AtIdentifier,
) -> Result<HashSet<Did>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    match actor {
        AtIdentifier::Did(did) => {
            found.insert(did.clone());
        }
        __ => {} // TODO: resolve and add
    }
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
            found.insert(f.did.clone());
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
) -> Result<HashSet<Did>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    match actor {
        AtIdentifier::Did(did) => {
            found.insert(did.clone());
        }
        __ => {} // TODO: resolve and add
    }
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
            found.insert(f.did.clone());
        }
        if cursor.is_none() {
            break;
        }
    }
    println!("got {} followed by {:?}", found.len(), actor);
    Ok(found)
}

pub async fn create_list(
    agent: &BskyAgent,
    name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = agent.get_session().await.expect("must be logged in");
    let record = api::app::bsky::graph::list::RecordData {
        name: name.clone(),
        purpose: MODLIST.to_string(),
        description: None,
        description_facets: None,
        labels: None,
        avatar: None,
        created_at: Datetime::now(),
    }
    .try_into_unknown()?;
    let res = agent
        .api
        .com
        .atproto
        .repo
        .create_record(
            api::com::atproto::repo::create_record::InputData {
                collection: api::app::bsky::graph::List::nsid(),
                record,
                repo: session.data.did.into(),
                rkey: None,
                swap_commit: None,
                validate: Some(true),
            }
            .into(),
        )
        .await?;
    println!("created new list by name {}, got {:?}", name, res);
    Ok(())
}

pub async fn get_lists(
    agent: &BskyAgent,
    actor: &AtIdentifier,
) -> Result<HashMap<String, ListViewData>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashMap::new();
    loop {
        let res = agent
            .api
            .app
            .bsky
            .graph
            .get_lists(
                api::app::bsky::graph::get_lists::ParametersData {
                    actor: actor.clone(),
                    cursor,
                    limit: None,
                }
                .into(),
            )
            .await?;
        cursor = res.cursor.clone();
        for list in res.lists.iter() {
            found.insert(list.data.name.clone(), list.data.clone());
        }
        if cursor.is_none() {
            break;
        }
    }
    Ok(found)
}

pub async fn get_users_on_list(
    agent: &BskyAgent,
    list: &String,
) -> Result<HashSet<Did>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashSet::new();
    loop {
        let res = agent
            .api
            .app
            .bsky
            .graph
            .get_list(
                api::app::bsky::graph::get_list::ParametersData {
                    list: list.clone(),
                    cursor,
                    limit: None,
                }
                .into(),
            )
            .await?;
        cursor = res.cursor.clone();
        for user in res.data.items.iter() {
            found.insert(user.data.subject.did.clone());
        }
        if cursor.is_none() {
            break;
        }
    }
    Ok(found)
}

pub async fn add_user_to_list(
    agent: &BskyAgent,
    list: &String,
    user: &Did,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Adding user {:?}", user);
    let session = agent.get_session().await.expect("must be logged in");
    let record = api::app::bsky::graph::listitem::RecordData {
        list: list.clone(),
        subject: user.clone(),
        created_at: Datetime::now(),
    }
    .try_into_unknown()?;
    agent
        .api
        .com
        .atproto
        .repo
        .create_record(
            api::com::atproto::repo::create_record::InputData {
                collection: api::app::bsky::graph::Listitem::nsid(),
                record,
                repo: session.data.did.into(),
                rkey: None,
                swap_commit: None,
                validate: Some(true),
            }
            .into(),
        )
        .await?;
    Ok(true)
}

pub async fn remove_user_from_list(
    _agent: &BskyAgent,
    _list: &String,
    user: &Did,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Removing user {:?}", user);
    todo!(); // need to implement this eventually
}
