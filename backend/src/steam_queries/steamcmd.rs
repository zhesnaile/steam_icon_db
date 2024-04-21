use std::io::{self, Write};
use std::process::{Command, Stdio};
use anyhow::Result;

fn get_client_icons(appid: u32) -> Result<()> {
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
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        println!(
            "SteamCMD command failed with exit status: {}",
            output.status
        );
    }

    Ok(())
}