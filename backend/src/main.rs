mod api;
mod steam_queries;

use crate::api::router::http_router_main;
use crate::steam_queries::update_game_db;
use anyhow::Result;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            match update_game_db().await {
                Ok(_) => (),
                Err(_err) => (),
            }
        }
    });

    http_router_main().await.unwrap();

    Ok(())
}
