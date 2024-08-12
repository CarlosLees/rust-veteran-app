use lib_core::AppConfig;
use mongodb::Database;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub database: Database,
    pub app_config: AppConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(database: Database, app_config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                database,
                app_config,
            }),
        }
    }
}
