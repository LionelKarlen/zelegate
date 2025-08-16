use std::process::{Command, ExitStatus};

use crate::config::{CustomSession, ZelegateConfig};

#[derive(Debug)]
enum SessionKind {
    Current,
    Alive,
    Exited,
    Custom,
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
}

#[derive(Debug)]
struct Session {
    name: String,
    kind: SessionKind,
}

pub fn show_list(config: &ZelegateConfig) {
    let sessions = list_sessions();
    let custom_sessions = list_custom_sessions(&config.session);

    sessions
        .iter()
        .for_each(|u| println!("{} ({:?})", u.name, u.kind));

    custom_sessions
        .iter()
        .for_each(|u| println!("{} ({:?})", u.name, u.kind));
}

fn list_custom_sessions(sessions: &Vec<CustomSession>) -> Vec<Session> {
    sessions
        .iter()
        .filter_map(|u| Some(Session::from_custom(u)))
        .collect()
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
