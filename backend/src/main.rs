use axum::{routing::get, Router};
use std::io::{self, Write};
use std::process::{Command, Stdio};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct App {
    appid: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppList {
    apps: Vec<App>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    applist: AppList,
}

fn call_steamcmd() -> Result<(), Box<dyn std::error::Error>> {
    let child = Command::new("steamcmd")
        .arg("+app_info_print")
        .arg("48190")
        .arg("+quit")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        println!("SteamCMD command executed successfully!");
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        println!(
            "SteamCMD command failed with exit status: {}",
            output.status
        );
    }

    Ok(())
}

async fn request_game_id_list() -> Result<(), Box<dyn std::error::Error>> {
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

async fn http_router_main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on port :{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    request_game_id_list().await.unwrap();
    //http_router_main().await.unwrap();

    Ok(())
}
