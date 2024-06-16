use anyhow::{anyhow, Result};
use redis::{Client, Commands};

use crate::config;
use crate::steam_queries::SteamApp;

pub async fn get_redis_client() -> Result<Client> {
    let config = config::get_redis_config();
    let address = format!("redis://{}:{}", config.host, config.port);
    let client =
        redis::Client::open(address).map_err(|e| anyhow!("Failed to connect to redis: {}", e))?;
    Ok(client)
}

pub async fn save_steam_apps(apps: &[SteamApp]) -> Result<()> {
    let client = get_redis_client().await?;

    let mut con = client
        .get_connection()
        .map_err(|e| anyhow!("Failed to get Redis connection: {}", e))?;

    for app in apps {
        let key = format!("steamapp:{}", app.appid);
        let value = serde_json::to_string(app)
            .map_err(|e| anyhow!("Failed to serialize SteamApp: {}", e))?;

        con.set(&key, &value)
            .map_err(|e| anyhow!("Failed to save SteamApp to Redis: {}", e))?;
    }

    Ok(())
}

pub async fn get_steam_app(app_id: u32) -> Result<Option<SteamApp>> {
    let client = get_redis_client().await?;

    let mut con = client
        .get_connection()
        .map_err(|e| anyhow!("Failed to get Redis connection: {}", e))?;

    let key = format!("steamapp:{}", app_id);
    let app_json: Option<String> = con
        .get(&key)
        .map_err(|e| anyhow!("Failed to retrieve SteamApp from Redis: {}", e))?;

    match app_json {
        Some(json) => Ok(serde_json::from_str(&json)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize SteamApp: {}", e))?),
        None => Ok(None),
    }
}
