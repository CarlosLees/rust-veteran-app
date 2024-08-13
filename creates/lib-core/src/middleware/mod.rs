use axum::{extract::State, http::Request};
use lib_entity::AppState;
use tracing::{error, info};


pub async fn mysql_pool_middleware<B>(
    State(state): State<AppState>,
    req: Request<B>) -> Request<B> {

    let headers = req.headers();
    let header_value = headers.get("imei")
            .and_then(|value| value.to_str().ok())
            .or_else(|| headers.get("companyid").and_then(|value| value.to_str().ok()));

    if let None = header_value {
        error!("not fount header");
    };
    info!("mongo_database:{:?}", state.mongo_database);
    req
    // 获取当前线程thread local的 mysql_pool 判断是否存在
}