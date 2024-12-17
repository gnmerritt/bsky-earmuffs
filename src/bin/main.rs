use bsky_sdk::BskyAgent;
use std::env;
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
    agent.login(config.auth.handle, password).await?;

    for list in config.lists {
        println!("Processing list {}", list.name);
        let _contents = earmuffs::resolve_blocklist(&agent, &list).await?;
    }

    Ok(())
}
