use axum::{extract::State, http::Request};
use lib_entity::{
    mongo::{Company, Device, ServerConfig},
    AppState,
};
use mongodb::bson::{doc, oid::ObjectId};
use tracing::info;
use urlencoding::encode;

use crate::{
    context::{get_map_mysql_pool, set_map_mysql_pool, set_mysql_pool},
    init_mysql_pool, POOL_MAP,
};

pub async fn mysql_pool_middleware<B>(
    State(state): State<AppState>,
    req: Request<B>,
) -> Request<B> {
    // 1. 判断dashmap中是否有对应server_config_id的pool
    // 2. 如果没有， 获取对应的pool 存入到dashmap和thread local中
    // 3. 如果有， 则直接存入到thread local中
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
                info!("company_id:{:?}", company_id);

                let company_collection = state.mongo_database.collection::<Company>("Company");
                let company_result: Result<Option<Company>, mongodb::error::Error> =
                    company_collection
                        .find_one(doc! {
                            "_id": ObjectId::parse_str(company_id).unwrap()
                        })
                        .await;
                info!("company_result:{:?}", company_result);

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

    if POOL_MAP.contains_key(&server_config_id) {
        // 已经有对应的pool
        let pool = get_map_mysql_pool(&server_config_id);
        if let Some(mysql_pool) = pool {
            set_mysql_pool(mysql_pool.to_owned())
        }
    } else {
        info!("serverConfigId:{:?}", server_config_id);
        if !server_config_id.is_empty() {
            let server_config_result: Result<Option<ServerConfig>, mongodb::error::Error> = state
                .mongo_database
                .collection("ServerConfig")
                .find_one(doc! {
                    "_id": ObjectId::parse_str(server_config_id.clone()).unwrap()
                })
                .await;
            info!("serverConfigResult:{:?}", server_config_result);
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
                    set_mysql_pool(pool.clone());
                    // 添加到dashmap
                    set_map_mysql_pool(server_config_id, pool)
                }
            }
        }
    }
    req
}
