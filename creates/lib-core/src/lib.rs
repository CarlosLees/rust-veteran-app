mod context;
mod error;
mod middleware;
mod mongo_config;
mod mysql_config;

pub use context::*;
pub use error::*;
pub use middleware::*;
pub use mongo_config::*;
pub use mysql_config::*;
use sqlx::MySqlPool;

pub fn get_mysql_pool_or_error() -> Result<MySqlPool, AppError> {
    match get_mysql_pool() {
        Some(pool) => Ok(pool),
        None => Err(AppError::ServiceError(String::from("未获取到设备信息"))),
    }
}
