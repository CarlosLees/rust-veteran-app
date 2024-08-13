pub mod mongo_config;
pub mod mysql_config;
pub mod middleware;
pub mod context;

use std::sync::Arc;
use dashmap::DashMap;
use sqlx::MySqlPool;

pub use crate::mysql_config::init_mysql_pool;
pub use middleware::*;

type PoolMap = DashMap<String, Arc<MySqlPool>>;


lazy_static::lazy_static!{
    static ref POOL_MAP: PoolMap = DashMap::new();
}

pub fn get_mysql_pool(key: &str) -> Option<Arc<MySqlPool>> {
    POOL_MAP.get(key).map(|eneity| eneity.value().clone())
}

pub fn set_mysql_pool(key: String, pool: Arc<MySqlPool>) {
    POOL_MAP.insert(key, pool);
} 