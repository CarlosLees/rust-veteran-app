use std::cell::RefCell;

use sqlx::MySqlPool;

thread_local! {
    static THREAD_MYSQL_POOL: RefCell<Option<MySqlPool>> = RefCell::new(None)
}

pub fn set_mysql_pool(pool: MySqlPool) {
    THREAD_MYSQL_POOL.with(|p| *p.borrow_mut() = Some(pool))
}

pub fn get_mysql_pool() -> Option<MySqlPool> {
    THREAD_MYSQL_POOL.with(|p| p.borrow().clone())
}
