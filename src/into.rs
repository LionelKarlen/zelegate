use std::{env, process::Command};

pub fn into(destination: &str) {
    println!("welcome to destination: {}", destination);

    // Check if we are in a zellij session
    let is_zellij = match env::var("ZELLIJ") {
        Ok(_s) => _s == "0",
        _ => false,
    };

    let mut command = Command::new("zellij");
    let child: &mut Command;

    match is_zellij {
        true => {
            println!("in zellij session");
            // zellij pipe --plugin zellij-switch -- "--session {session_name} --cwd $HOME/{session_path}"
            child = command
                .arg("pipe")
                .arg("--plugin")
                .arg("zellij-switch")
                .arg("--")
                .arg(format!("--session {destination}"));
        }
        false => {
            println!("not in zellij session");
            // zellij attach -c {session_name}
            child = command.arg("attach").arg(destination);
        }
    }

    println!("starting command");

    // Use status here, because it just lets the command run
    let _ = child.status().expect("Failed to start command");
    println!("terminating.");
}
