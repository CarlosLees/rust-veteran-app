use anyhow::Result;
use axum::middleware::map_request_with_state;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{serve, Router};
use lib_core::{get_mysql_pool_or_error, init_mongo_client, mysql_pool_middleware, AppError};
use lib_entity::mysql::LitemallInfoVeteran;
use lib_entity::AppState;
use lib_utils::{AppConfig, HttpResult};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = AppConfig::try_load()?;
    let mongo_database = init_mongo_client(&app_config.mongo.url, &app_config.mongo.db_name).await;

    let app_state = AppState::new(mongo_database, app_config);

    let addr = format!("0.0.0.0:{}", app_state.app_config.server.port);
    println!("Listening on {}", addr);

    let listen = TcpListener::bind(&addr).await?;

    let app = Router::new()
        .route("/version", get(get_version_handler))
        .route_layer(map_request_with_state(
            app_state.clone(),
            mysql_pool_middleware,
        ))
        // .layer(middleware::from_fn(check_pool_connection))
        .with_state(app_state);

    serve(listen, app.into_make_service()).await?;
    Ok(())
}

async fn get_version_handler() -> Result<impl IntoResponse, AppError> {
    let pool = get_mysql_pool_or_error()?;

    let veteran_list: Vec<LitemallInfoVeteran> =
        sqlx::query_as(r#"select * from litemall_info_veteran where deleted = ?"#)
            .bind(false)
            .fetch_all(&pool)
            .await
            .map_err(|_| AppError::ServiceError(String::from("Database query failed")))?;

    Ok(HttpResult::ok(veteran_list))
}
