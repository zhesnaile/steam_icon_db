use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Deserialize, Debug, Serialize)]
struct AppInfo {
    common: ClientIcons,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ClientIcons {
    clienticon: Option<String>,
    linuxclienticon: Option<String>,
}

pub fn get_client_icons(appid: u32) -> Result<ClientIcons> {
    let child = Command::new("steamcmd")
        .arg("+app_info_print")
        .arg(appid.to_string())
        .arg("+quit")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        println!("SteamCMD command executed successfully!");

        let d = String::from_utf8(output.stdout).unwrap();

        let mut steam_app_vec = Vec::new();
        let mut pattern_found = false;

        let pattern = format!("\"{appid}\"");

        for line in d.lines() {
            if !pattern_found {
                if line == pattern {
                    pattern_found = true;
                    steam_app_vec.push(line.to_string());
                }
            } else {
                steam_app_vec.push(line.to_string());
            }
        }

        let steam_app_vec = steam_app_vec.join("");

        let app_info: AppInfo = keyvalues_serde::from_str(&steam_app_vec)?;

        return Ok(app_info.common);
    } else {
        return Err(anyhow!(
            "SteamCMD command failed with exit status: {}",
            output.status
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_icons() {
        const APPID: u32 = 590380;

        let clienticon = Some(String::from("d9157d92d45689e1ec92aea00980fcfad0ce977e"));
        let linuxclienticon = None;

        let result = get_client_icons(APPID).unwrap();

        assert_eq!(
            (clienticon, linuxclienticon),
            (result.clienticon, result.linuxclienticon)
        )
    }
}
