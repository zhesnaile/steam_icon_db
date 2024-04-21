mod api;
mod steam_queries;

use tokio::time;
use crate::api::router::http_router_main;
use crate::steam_queries::gamelist::request_game_id_list;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            match request_game_id_list().await {
                Ok(game_list) => println!("{:?}", game_list),
                Err(err) => eprintln!("{}", err)
            }
        }
    });

    http_router_main().await.unwrap();

    Ok(())
}
