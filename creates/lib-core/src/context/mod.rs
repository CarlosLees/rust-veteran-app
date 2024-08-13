use std::cell::RefCell;

use sqlx::MySqlPool;

thread_local! {
    static THREAD_MYSQL_POOL: RefCell<Option<MySqlPool>> = RefCell::new(None)
}
