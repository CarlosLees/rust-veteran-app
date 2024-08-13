use lib_utils::AppConfig;
use mongodb::Database;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub mongo_database: Database,
    pub app_config: AppConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(mongo_database: Database, app_config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                mongo_database,
                app_config,
            }),
        }
    }
}
