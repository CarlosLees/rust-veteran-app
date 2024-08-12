use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_ip: String,
    pub user_name: String,
    pub pass: String,
    pub port: u16,
    pub dbname: String,
    pub add_time: DateTime,
}
