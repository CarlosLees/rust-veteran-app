use axum::{http::Method, middleware::map_request_with_state, routing::get, Router};
use lib_core::mysql_pool_middleware;
use lib_entity::AppState;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    get_version_handler,
    handlers::{car_record_list_handler, company_list},
};

pub async fn get_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/version", get(get_version_handler))
        .route("/company_list", get(company_list))
        .route("/car", get(car_record_list_handler))
        .route_layer(map_request_with_state(
            app_state.clone(),
            mysql_pool_middleware,
        ))
        .layer(cors)
        .with_state(app_state);
    app
}
