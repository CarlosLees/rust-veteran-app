use crate::config::AppState;
use anyhow::Result;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{serve, Json, Router};
use lib_core::mongo_config::init_mongo_client;
use lib_core::{init_mysql_pool, AppConfig};
use lib_entity::mongo::ServerConfig;
use lib_entity::mysql::LitemallInfoVeteran;
use mongodb::bson::doc;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;
use urlencoding::encode;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = AppConfig::try_load()?;
    let database = init_mongo_client(&app_config.mongo.url, &app_config.mongo.db_name).await;

    let app_state = AppState::new(database, app_config);

    let addr = format!("0.0.0.0:{}", app_state.app_config.server.port);
    println!("Listening on {}", addr);

    let listen = TcpListener::bind(&addr).await?;

    let app = Router::new()
        .route("/version", get(get_version_handler))
        // .layer(axum::middleware::from_fn(pg_pool_middleware))
        .with_state(app_state);

    serve(listen, app.into_make_service()).await?;
    Ok(())
}

async fn get_version_handler(State(state): State<AppState>) -> impl IntoResponse {
    let one: Option<ServerConfig> = state
        .inner
        .database
        .collection("ServerConfig")
        .find_one(doc! {})
        .await
        .unwrap();
    if let Some(config) = one {
        // 连接到mysql
        let pool = init_mysql_pool(
            format!(
                "mysql://{}:{}@{}:{}/{}",
                config.user_name,
                encode(config.pass.as_str()),
                config.server_ip,
                config.port,
                config.dbname
            )
            .as_str(),
        )
        .await
        .unwrap();
        let veteran_list: Vec<LitemallInfoVeteran> =
            sqlx::query_as("select * from litemall_info_veteran")
                .fetch_all(&pool)
                .await
                .unwrap();
        return Json(veteran_list);
    }
    Json(vec![])
}
