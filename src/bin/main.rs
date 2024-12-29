use bsky_sdk::BskyAgent;
use earmuffs::{
    bsky::{self, get_users_on_list},
    EarmuffsConfig,
};
use std::env;
use tokio;

async fn get_ro_agent(config: &EarmuffsConfig) -> Result<BskyAgent, Box<dyn std::error::Error>> {
    let agent = BskyAgent::builder().build().await?;
    let handle = env::var("EM_RO_HANDLE");
    let password = match handle {
        Ok(_) => {
            println!("Found a RO handle in the environment, using it");
            env::var("EM_RO_APP_PW")
                .expect("need a read-only app password in $EM_RO_APP_PW to go with the RO handle")
        }
        Err(_) => config.auth.app_password.clone().unwrap_or_else(|| {
            env::var("EM_APP_PW").expect("Need a password in json or environment var $EM_APP_PW")
        }),
    };
    let handle = handle.unwrap_or(config.auth.handle.clone().into());

    agent.login(&handle, password).await?;
    Ok(agent)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config_file = env::var("EM_FILE").unwrap_or("earmuffs.json".to_string());
    let config = earmuffs::read_config(config_file)?;
    // optionally scrape data using a separate account from the list owner
    let ro_agent = get_ro_agent(&config).await?;
    // the modlist host account must be used to make edits
    let list_owner = BskyAgent::builder().build().await?;
    let password = config.auth.app_password.clone().unwrap_or_else(|| {
        env::var("EM_APP_PW").expect("Need a password in json or environment var $EM_APP_PW")
    });
    list_owner.login(&config.auth.handle, password).await?;

    let user_lists = bsky::get_lists(&ro_agent, &config.auth.handle).await?;

    for spec in config.lists {
        println!("Processing list '{}'", spec.name);
        if !user_lists.contains_key(&spec.name) {
            bsky::create_list(&ro_agent, &spec.name).await?;
            println!("List created, need to wait for it to appear so skipping for now");
            continue;
        }
        let list = user_lists
            .get(&spec.name)
            .expect(&format!("list '{}' was not created", spec.name));
        let current_users = get_users_on_list(&ro_agent, &list.uri).await?;
        let current_user_dids = current_users.keys().cloned().collect();
        let users_on_list = earmuffs::resolve_blocklist(&ro_agent, &spec).await?;
        println!(
            "list currently contains {} users, will have {} after updating",
            current_users.len(),
            users_on_list.len(),
        );

        let to_add = users_on_list.difference(&current_user_dids);
        for user in to_add {
            bsky::add_user_to_list(&list_owner, &list.uri, user).await?;
        }
        // remove anyone who doesn't meet the list criteria (TODO add a delay here eventually)
        let to_remove = current_user_dids.difference(&users_on_list);
        for user in to_remove {
            let uri = current_users.get(user);
            if let Some(uri) = uri {
                bsky::remove_user_from_list(&list_owner, user, uri).await?;
            }
        }
    }

    Ok(())
}
