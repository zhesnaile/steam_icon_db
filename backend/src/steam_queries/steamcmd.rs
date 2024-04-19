use std::io::{self, Write};
use std::process::{Command, Stdio};

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