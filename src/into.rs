use std::{env, process::Command};

use crate::{
    config::{CustomSession, ZelegateConfig},
    list::SessionKind,
};

struct IntoSession {
    pub name: String,
    pub path: String,
    pub kind: SessionKind,
}

impl IntoSession {
    fn new(name: String, kind: String, config: &ZelegateConfig) -> Self {
        let session_kind = SessionKind::from_string(&kind);
        let mut session_name = name.clone();
        let mut session_path = &config.general.default_path;

        match session_kind {
            SessionKind::Folder => {
                session_name = name
                    .split("/")
                    .last()
                    .expect("failed to parse name")
                    .replace(" ", "_");
                session_path = &name;
            }
            SessionKind::Custom => {
                let custom_session: &&CustomSession = &config
                    .session
                    .iter()
                    .find(|u| u.name == session_name)
                    .expect("not found");
                session_path = &custom_session.path;
            }
            _ => {}
        }

        Self {
            name: session_name.replace(" ", "_"),
            path: session_path.to_string(),
            kind: session_kind,
        }
    }
}

pub fn into(destination: &str, config: &ZelegateConfig) {
    println!("welcome to destination: {}", destination);

    if destination.len() == 0 {
        return;
    }

    // Check if we are in a zellij session
    let is_zellij = match env::var("ZELLIJ") {
        Ok(_s) => _s == "0",
        _ => false,
    };

    let mut command: Command;
    let child: &mut Command;

    let (session_name, session_kind) = destination
        .split_once(" ")
        .unwrap_or((destination, "(Unknown)"));

    let session = IntoSession::new(session_name.to_string(), session_kind.to_string(), &config);

    match session.kind {
        SessionKind::Current => return,
        SessionKind::Exited => {}
        _ => {}
    }

    match is_zellij {
        true => {
            // zellij pipe --plugin zellij-switch -- "--session {session_name} --cwd $HOME/{session_path}"
            command = Command::new("zellij");
            println!("in session");
            child = command
                .arg("pipe")
                .arg("--plugin")
                .arg("zellij-switch")
                .arg("--")
                .arg(format!("--session {} --cwd {}", session.name, session.path))
        }
        false => {
            // zellij attach -c {session_name}
            dbg!(&session.path);
            command = Command::new("zellij");
            child = command
                .current_dir(session.path)
                .arg("attach")
                .arg("-c")
                .arg(session.name);
        }
    }

    // Use status here, because it just lets the command run
    let _ = child.status().expect("Failed to start command");
    println!("terminating.");
}
