use axum::{middleware::map_request_with_state, routing::get, Router};
use lib_core::mysql_pool_middleware;
use lib_entity::AppState;

use crate::get_version_handler;

pub async fn get_router(app_state: AppState) -> Router {
    let app = Router::new()
        .route("/version", get(get_version_handler))
        .route_layer(map_request_with_state(
            app_state.clone(),
            mysql_pool_middleware,
        ))
        .with_state(app_state);
    app
}
