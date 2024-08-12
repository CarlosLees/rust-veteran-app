use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mongo: MongoConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoConfig {
    pub url: String,
    pub db_name: String,
}

impl AppConfig {
    pub fn try_load() -> Result<Self> {
        let result = match (
            File::open("app.yml"),
            File::open("/etc/server_config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config Not Found"),
        };
        Ok(result?)
    }
}
