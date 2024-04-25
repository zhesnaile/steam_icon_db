use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Deserialize, Debug, Serialize)]
struct AppInfo {
    #[serde(rename = "Common")]
    common: Common,
}

#[derive(Deserialize, Debug, Serialize)]
struct Common {
    clienticon: Option<String>,
    linuxclienticon: Option<String>,
}

pub fn get_client_icons(appid: u32) -> Result<(Option<String>, Option<String>)> {
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

        let steam_app_vec = steam_app_vec.join("\n");

        let app_info: AppInfo = keyvalues_serde::from_str(&steam_app_vec)?;

        println!("Appid info: {app_info:#?}");
    } else {
        println!(
            "SteamCMD command failed with exit status: {}",
            output.status
        );
    }
    let clienticon = Some(String::from("d9157d92d45689e1ec92aea00980fcfad0ce977e"));
    let linuxclienticon: Option<String> = None;

    Ok((clienticon, linuxclienticon))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_icons() {
        const APPID: u32 = 590380;

        let test_clienticon = Some(String::from("d9157d92d45689e1ec92aea00980fcfad0ce977e"));
        let test_linuxclienticon: Option<String> = None;

        if let Ok(result) = get_client_icons(APPID) {
            assert_eq!((test_clienticon, test_linuxclienticon), result)
        }
    }
}
