use axum::{extract::State, http::Request};
use lib_entity::{
    mongo::{Company, Device, ServerConfig},
    AppState,
};
use mongodb::bson::doc;
use sqlx::pool;
use tracing::info;
use urlencoding::encode;

use crate::{
    context::{get_mysql_pool, set_mysql_pool},
    init_mysql_pool,
};

pub async fn mysql_pool_middleware<B>(
    State(state): State<AppState>,
    req: Request<B>,
) -> Request<B> {
    let headers = req.headers();
    let imei_header = headers.get("imei").and_then(|value| value.to_str().ok());
    let server_config_id: String = match imei_header {
        Some(imei) => {
            // 根据imei获取serverConfigId
            let device_result: Result<Option<Device>, mongodb::error::Error> = state
                .mongo_database
                .collection("Device")
                .find_one(doc! {
                    "imei": imei
                })
                .await;
            if let Ok(Some(device)) = device_result {
                device.server_config_id
            } else {
                String::new()
            }
        }
        None => {
            let company_id_header = headers
                .get("companyid")
                .and_then(|value| value.to_str().ok());

            if let Some(company_id) = company_id_header {
                let company_result: Result<Option<Company>, mongodb::error::Error> = state
                    .mongo_database
                    .collection("Company")
                    .find_one(doc! {
                        "_id": company_id
                    })
                    .await;
                if let Ok(Some(company)) = company_result {
                    company.server_config_id
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
    };
    // 获取当前线程thread local的 mysql_pool 判断是否存在
    if let None = get_mysql_pool() {
        info!("未获取到对应的pool");
        // 获取serverConfig
        info!("serverConfigId:{:?}", server_config_id);
        if !server_config_id.is_empty() {
            let server_config_result: Result<Option<ServerConfig>, mongodb::error::Error> = state
                .mongo_database
                .collection("ServerConfig")
                .find_one(doc! {
                    "_id": server_config_id
                })
                .await;
            if let Ok(Some(server_config)) = server_config_result {
                if let Ok(pool) = init_mysql_pool(
                    format!(
                        "mysql://{}:{}@{}:{}/{}",
                        server_config.user_name,
                        encode(server_config.pass.as_str()),
                        server_config.server_ip,
                        server_config.port,
                        server_config.dbname
                    )
                    .as_str(),
                )
                .await
                {
                    // 添加到thread_local
                    set_mysql_pool(pool);
                }
            }
        }
    }

    req
}
