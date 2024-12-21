use std::collections::{HashMap, HashSet};

use bsky_sdk::{
    api::{
        self,
        app::bsky::graph::defs::{ListViewData, MODLIST},
        types::string::{AtIdentifier, Datetime, Did, Handle},
    },
    BskyAgent,
};

pub(crate) async fn resolve_handle(
    agent: &BskyAgent,
    actor: &Handle,
) -> Result<Did, Box<dyn std::error::Error>> {
    let res = agent
        .api
        .com
        .atproto
        .identity
        .resolve_handle(
            api::com::atproto::identity::resolve_handle::ParametersData {
                handle: actor.clone(),
            }
            .into(),
        )
        .await?;
    Ok(res.did.clone())
}

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
        AtIdentifier::Handle(h) => {
            let did = resolve_handle(agent, h).await?;
            found.insert(did);
        }
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
        AtIdentifier::Handle(h) => {
            let did = resolve_handle(agent, h).await?;
            found.insert(did);
        }
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
    let record = api::app::bsky::graph::list::RecordData {
        name: name.clone(),
        purpose: MODLIST.to_string(),
        description: None,
        description_facets: None,
        labels: None,
        avatar: None,
        created_at: Datetime::now(),
    };
    let res = agent.create_record(record).await?;
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
) -> Result<HashMap<Did, String>, Box<dyn std::error::Error>> {
    let mut cursor = None;
    let mut found = HashMap::new();
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
            found.insert(user.data.subject.did.clone(), user.uri.clone());
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
    let record = api::app::bsky::graph::listitem::RecordData {
        list: list.clone(),
        subject: user.clone(),
        created_at: Datetime::now(),
    };
    agent.create_record(record).await?;
    Ok(true)
}

pub async fn remove_user_from_list(
    agent: &BskyAgent,
    user: &Did,
    uri: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Removing user {:?} at {}", user, uri);
    agent.delete_record(uri).await?;
    Ok(true)
}
