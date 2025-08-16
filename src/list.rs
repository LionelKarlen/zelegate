use std::process::{Command, ExitStatus};

#[derive(Debug)]
enum SessionKind {
    Current,
    Alive,
    Exited,
    Unknown,
}

impl Session {
    fn new(s: &str) -> Self {
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
}

#[derive(Debug)]
struct Session {
    name: String,
    kind: SessionKind,
}

pub fn show_list() {
    list_sessions();
}

fn list_sessions() {
    let value = Command::new("zellij").arg("ls").arg("-n").output();
    // TODO: [ERROR] handle error properly
    let value = value.expect("failed to run zellij command");
    if ExitStatus::success(&value.status) {
        let sessions = str::from_utf8(&value.stdout).unwrap();
        let sessions: Vec<Session> = sessions.lines().map(|u| Session::new(u)).collect();
        sessions
            .iter()
            .for_each(|u| println!("{} ({:?})", u.name, u.kind));
    } else {
        // TODO: [ERROR] handle error properly
        panic!("Zellij list command failed.");
    }
}
