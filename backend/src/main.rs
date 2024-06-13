mod api;
mod steam_queries;

use crate::api::router::http_router_main;
use crate::steam_queries::update_game_db;
use anyhow::Result;
use tokio::time;

async fn update_game_db_task() {
    let mut interval = time::interval(time::Duration::from_secs(60));
    loop {
        interval.tick().await;
        match update_game_db().await {
            Ok(_) => (),
            Err(_err) => (),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(update_game_db_task());

    http_router_main().await.unwrap();

    Ok(())
}
