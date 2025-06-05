use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    pub chucks: std::collections::HashMap<String, Chuck>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_daemon_mode")]
    pub daemon_mode: bool,
    #[serde(default = "default_notification_timeout")]
    pub notification_timeout: u32,
}

#[derive(Debug, Deserialize)]
pub struct Chuck {
    pub keys: Vec<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub webhook: Option<String>,
    #[serde(default)]
    pub sound: Option<PathBuf>,
    #[serde(default)]
    pub ascii: Option<PathBuf>,
    #[serde(default)]
    pub notification: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    #[serde(default = "default_retry_count")]
    pub retry_count: u32,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_daemon_mode() -> bool {
    true
}

fn default_notification_timeout() -> u32 {
    2000
}

fn default_timeout() -> u32 {
    5000
}

fn default_retry_count() -> u32 {
    3
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}
