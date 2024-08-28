use anyhow::Result;
use axum::{extract::Query, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Utc};
use lib_core::{get_mysql_pool_or_error, AppError};
use lib_utils::{serialize_datetime, serialize_datetime_with_option, HttpResult};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, QueryBuilder};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CarRecordListParam {
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
    pub veteran_id: i32,
    pub veteran_name: String,
    pub addr: String,
    #[serde(serialize_with = "serialize_datetime_with_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_datetime")]
    pub add_time: DateTime<Utc>,
    pub car_unit: Option<String>,
    pub car_use: Option<String>,
    pub start_addr: Option<String>,
    pub driver_name: Option<String>,
    pub filler: Option<String>,
    pub stopwatch_start: Option<String>,
    pub stopwatch_end: Option<String>,
    #[serde(serialize_with = "serialize_datetime_with_option")]
    pub return_time: Option<DateTime<Utc>>,
    pub is_return: i32,
}

pub async fn car_record_list_handler(
    Query(param): Query<CarRecordListParam>,
) -> Result<impl IntoResponse, AppError> {
    let pool = get_mysql_pool_or_error()?;

    let mut query_builder = QueryBuilder::new(
        r#"select * from litemall_car_record WHERE be_batch = false And deleted = false"#,
    );

    if let Some(license) = param.license {
        query_builder.push(" AND license = ").push_bind(license);
    }

    if let Some(last_id) = param.last_id {
        query_builder.push(" AND id > ").push_bind(last_id);
    }

    query_builder
        .push(" order by id desc limit ")
        .push_bind(param.limit);

    let result: Vec<CarRecordListResponse> =
        query_builder.build_query_as().fetch_all(&pool).await?;

    Ok(HttpResult::ok(result))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateCardRecordParam {
    pub id: i32,
    pub start_time: String,
    pub stopwatch_start: String,
    pub stopwatch_end: String,
    pub return_time: String,
}

pub async fn update_car_record(
    WithRejection(Json(param), _): WithRejection<Json<UpdateCardRecordParam>, AppError>,
) -> Result<impl IntoResponse, AppError> {
    // WithRejection 在解析json参数时，如果解析失败，可以返回自定义错误，而非axum返回的字符串
    let pool = get_mysql_pool_or_error()?;

    let result: Option<CarRecordListResponse> = sqlx::query_as("update litemall_car_record set start_time = ?, stopwatch_start = ?,stopwatch_end = ?,return_time = ? where id = ?")
            .bind(param.start_time)
            .bind(param.stopwatch_start)
            .bind(param.stopwatch_end)
            .bind(param.return_time)
            .bind(param.id)
            .fetch_optional(&pool).await?;
    Ok(HttpResult::ok(result))
}
