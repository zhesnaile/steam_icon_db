use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::{gamelist::request_game_id_list, steamcmd::get_client_icons};
pub mod gamelist;
pub mod steamcmd;

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamApp {
    pub appid: u32,
    pub name: String,
    pub linuxclienticon: Option<String>,
    pub clienticon: Option<String>,
}

pub async fn update_game_db() -> Result<()> {
    let game_list = request_game_id_list().await?;

    for steamapp in game_list {
        if !steamapp.name.trim().is_empty() {
            match get_client_icons(steamapp.appid).await {
                Ok(val) => println!("Appid {0:?}: {1:?}", steamapp.appid, val.clienticon),
                Err(err) => eprintln!("{}", err),
            };
        }
    }
    Ok(())
}
