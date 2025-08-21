use std::process::{Command, ExitStatus};

use crate::config::{CustomSession, ZelegateConfig};

#[derive(Debug)]
enum SessionKind {
    Current,
    Alive,
    Exited,
    Custom,
    Folder,
    Unknown,
}

impl Session {
    fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let name = *parts.first().unwrap();
        let kind = *parts.last().unwrap();
        let kind = match kind {
            "(current)" => SessionKind::Current,
            "ago]" => SessionKind::Alive,
            "resurrect)" => SessionKind::Exited,
            _ => SessionKind::Unknown,
        };

        Session {
            name: name.to_string(),
            kind: kind,
        }
    }

    fn from_custom(c: &CustomSession) -> Self {
        Self {
            name: c.name.to_string(),
            kind: SessionKind::Custom,
        }
    }

    fn from_folder(path_string: &str) -> Self {
        Self {
            name: path_string.to_string(),
            kind: SessionKind::Folder,
        }
    }
}

#[derive(Debug)]
struct Session {
    name: String,
    kind: SessionKind,
}

pub fn show_list(config: &ZelegateConfig) {
    let sessions = list_sessions();
    let custom_sessions = list_custom_sessions(&config.session);
    let folder_sessions = list_folders(&config);

    sessions
        .iter()
        .for_each(|u| println!("{} ({:?})", u.name, u.kind));

    custom_sessions
        .iter()
        .for_each(|u| println!("{} ({:?})", u.name, u.kind));

    folder_sessions
        .iter()
        .for_each(|u| println!("{} ({:?})", u.name, u.kind));
}

fn list_custom_sessions(sessions: &Vec<CustomSession>) -> Vec<Session> {
    sessions
        .iter()
        .filter_map(|u| Some(Session::from_custom(u)))
        .collect()
}

fn list_folders(config: &ZelegateConfig) -> Vec<Session> {
    if config.general.folder_backend.len() > 0 {
        let args = config.general.folder_backend.clone();
        // TODO: [ERROR] handle error properly
        let (cmd, args) = args.split_once(' ').unwrap();
        let output = Command::new(cmd).args(args.split_whitespace()).output();
        // TODO: [ERROR] handle error properly
        let output = output.expect("failed to run command");

        if ExitStatus::success(&output.status) {
            let s = str::from_utf8(&output.stdout).unwrap();
            let sessions: Vec<Session> = s.lines().map(|u| Session::from_folder(u)).collect();
            return sessions;
        }
        panic!("failed to run folder comand");
    }
    return vec![];
}

fn list_sessions() -> Vec<Session> {
    let value = Command::new("zellij").arg("ls").arg("-n").output();
    // TODO: [ERROR] handle error properly
    let value = value.expect("failed to run zellij command");
    if ExitStatus::success(&value.status) {
        let sessions = str::from_utf8(&value.stdout).unwrap();
        let sessions: Vec<Session> = sessions.lines().map(|u| Session::from_string(u)).collect();
        sessions
    } else {
        // TODO: [ERROR] handle error properly
        panic!("Zellij list command failed.");
    }
}
