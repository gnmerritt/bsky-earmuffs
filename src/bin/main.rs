use bsky_sdk::BskyAgent;
use earmuffs::bsky::{self, get_users_on_list};
use std::{env, thread, time::Duration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config_file = env::var("EM_FILE").unwrap_or("earmuffs.json".to_string());
    let config = earmuffs::read_config(config_file)?;
    let agent = BskyAgent::builder().build().await?;
    let password = config.auth.app_password.unwrap_or_else(|| {
        env::var("EM_APP_PW").expect("Need a password in json or environment var $EM_APP_PW")
    });
    agent.login(&config.auth.handle, password).await?;

    let user_lists = bsky::get_lists(&agent, &config.auth.handle).await?;

    for spec in config.lists {
        println!("Processing list '{}'", spec.name);
        if !user_lists.contains_key(&spec.name) {
            bsky::create_list(&agent, &spec.name).await?;
            println!("List created, need to wait for it to appear so skipping for now");
            continue;
        }
        let list = user_lists
            .get(&spec.name)
            .expect(&format!("list '{}' was not created", spec.name));
        let current_users = get_users_on_list(&agent, &list.uri).await?;
        let current_user_dids = current_users.keys().cloned().collect();
        let users_on_list = earmuffs::resolve_blocklist(&agent, &spec).await?;
        println!(
            "list currently contains {} users, will have {} after updating",
            current_users.len(),
            users_on_list.len(),
        );

        let to_add = users_on_list.difference(&current_user_dids);
        for user in to_add {
            bsky::add_user_to_list(&agent, &list.uri, user).await?;
        }
        // remove anyone who has unfollowed him (TODO add a delay here eventually)
        let to_remove = current_user_dids.difference(&users_on_list);
        for user in to_remove {
            let uri = current_users.get(user);
            if let Some(uri) = uri {
                bsky::remove_user_from_list(&agent, user, uri).await?;
            }
        }
    }

    Ok(())
}
