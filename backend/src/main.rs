mod api;
mod steam_queries;

use api::router::http_router_main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //request_game_id_list().await.unwrap();
    crate::api::router::http_router_main().await.unwrap();

    Ok(())
}
