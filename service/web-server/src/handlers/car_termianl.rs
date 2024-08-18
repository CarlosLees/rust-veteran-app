use axum::{extract::Query, response::IntoResponse};
use chrono::{DateTime, Utc};
use lib_core::{get_mysql_pool_or_error, AppError};
use lib_utils::HttpResult;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, QueryBuilder};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CarRecordParam {
    pub limit: i32,
    pub license: Option<String>,
    #[serde(rename = "lastId")]
    pub last_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CarRecordListResponse {
    pub id: Option<i32>,
    pub license: String,
    pub veteran_id: u32,
    pub veteran_name: String,
    pub addr: String,
    pub start_time: DateTime<Utc>,
    pub add_time: DateTime<Utc>,
    pub car_unit: String,
    pub car_use: String,
    pub start_addr: String,
    pub driver_name: String,
    pub filter: String,
    pub stop_watch_start: String,
    pub stop_watch_end: String,
    pub return_time: String,
    pub is_return: u32,
}

pub async fn car_record_list_handler(
    Query(param): Query<CarRecordParam>,
) -> Result<impl IntoResponse, AppError> {
    let pool = get_mysql_pool_or_error()?;

    let mut query_builder =
        QueryBuilder::new(r#"select * from litemall_car_record WHERE be_batch = false"#);

    if let Some(license) = param.license.clone() {
        query_builder.push(" AND license = ").push_bind(license);
    }

    if let Some(last_id) = param.last_id.clone() {
        query_builder.push(" AND id > ").push_bind(last_id);
    }

    query_builder
        .push(" order by id desc limit ")
        .push_bind(param.limit);

    let result: Vec<CarRecordListResponse> =
        query_builder.build_query_as().fetch_all(&pool).await?;

    Ok(HttpResult::ok(result))
}
