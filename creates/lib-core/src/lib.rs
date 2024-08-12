pub mod mongo_config;
pub mod mysql_config;
pub mod server_config;

pub use crate::mysql_config::init_mysql_pool;
pub use crate::server_config::app_config::AppConfig;
