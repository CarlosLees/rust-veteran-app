use sqlx::{Error, MySqlPool};

pub async fn init_mysql_pool(database_url: &str) -> Result<MySqlPool, Error> {
    MySqlPool::connect(database_url).await
}
