use serde::Deserialize;
use std::{fs::read_to_string, io::ErrorKind, path::PathBuf};

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ZelegateConfig {
    pub general: GeneralConfig,
    pub session: Vec<CustomSession>,
}

impl Default for ZelegateConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            session: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct GeneralConfig {
    pub prefer_custom: bool,
    pub prefer_create: bool,
    pub default_layout: String,
    pub folder_backend: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            prefer_custom: true,
            prefer_create: true,
            default_layout: "default".to_string(),
            folder_backend: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct CustomSession {
    pub name: String,
    pub path: PathBuf,
    pub layout: String,
}
impl Default for CustomSession {
    fn default() -> Self {
        Self {
            name: Default::default(),
            path: "$HOME".into(),
            layout: "default".to_string(),
        }
    }
}
impl ZelegateConfig {
    fn from_str(s: &str) -> Self {
        // TODO: [ERROR] Handle error properly
        toml::from_str(s).expect("parse error")
    }
}

pub fn read_config(path: PathBuf) -> ZelegateConfig {
    match read_to_string(path) {
        Ok(s) => ZelegateConfig::from_str(&s),
        Err(e) => {
            match e.kind() {
                // If the configuration file cannot be found, we simply use the default
                // configuration. This will likely mainly happen if the file
                // `~/.config/zelegate/zelegate.toml` doesn't exist
                ErrorKind::NotFound => ZelegateConfig::default(),
                _ => {
                    // TODO: [ERROR] Handle error properly
                    panic!("config error: {}", e);
                }
            }
        }
    }
}
