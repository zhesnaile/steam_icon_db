use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct App {
    appid: u32,
    name: String,
    linuxclienticon: Option<String>,
    clienticon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppList {
    apps: Vec<App>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    applist: AppList,
}

pub async fn request_game_id_list() -> Result<(), Box<dyn std::error::Error>> {
    const STEAM_GET_APP_LIST_URL: &str = "https://api.steampowered.com/ISteamApps/GetAppList/v2/";
    match reqwest::get(STEAM_GET_APP_LIST_URL).await {
        Ok(resp) => {
            let json: Response = resp.json().await?;
            let game_map: HashMap<u32, String> = json
                .applist
                .apps
                .into_iter()
                .map(|app| (app.appid, app.name))
                .collect();

            println!("{:?}", game_map);
        }
        Err(err) => {
            println!("Reqwest Error: {}", err);
        }
    }
    Ok(())
}