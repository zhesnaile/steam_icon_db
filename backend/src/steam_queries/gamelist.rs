use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::steam_queries::SteamApp;

#[derive(Debug, Deserialize, Serialize)]
struct SteamAppList {
    apps: Vec<SteamApp>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SteamAPIResponse {
    applist: SteamAppList,
}

pub async fn request_game_id_list() -> Result<Vec<SteamApp>> {
    const STEAM_GET_APP_LIST_URL: &str = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";
    let gamelist = reqwest::get(STEAM_GET_APP_LIST_URL)
        .await
        .context("Failed get request")?
        .json::<SteamAPIResponse>()
        .await
        .context("Failed to get JSON body")?
        .applist
        .apps;

    Ok(gamelist)
}
