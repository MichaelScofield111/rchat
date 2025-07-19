use std::{env, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    // pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // read from ./app.yml or /etc/config/app.yml or from env CHAT_CONFIG
        // "The match expression evaluates all three configuration attempts
        // simultaneously and then checks their results."
        let ret: Result<AppConfig, _> = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(file), _, _) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => anyhow::bail!("Failed to load config"),
        };

        Ok(ret?)
    }
}
