use axum::{
    extract::rejection::JsonRejection,
    http::{header, StatusCode},
    response::IntoResponse,
};
use lib_utils::HttpResult;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
    #[error("{0}")]
    ServiceError(String),
    #[error("sql error")]
    SqlXError(#[from] sqlx::Error),
    #[error("mongo error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("parse error")]
    ParseError(#[from] chrono::ParseError),
    #[error("axum error")]
    AxumError(#[from] axum::Error),
    #[error("json deserialize error")]
    JsonDeserializeError(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                HttpResult::<String>::error("not found".into()),
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                HttpResult::<String>::error("UNAUTHORIZED".into()),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                HttpResult::<String>::error("Internal server error".into()),
            ),
            AppError::ServiceError(error_str) => {
                (StatusCode::OK, HttpResult::<String>::error(error_str))
            }
            AppError::SqlXError(sql_error) => {
                error!("sql error:{:?}", sql_error);
                (
                    StatusCode::OK,
                    HttpResult::<String>::error("查询失败".into()),
                )
            }
            AppError::MongoError(mongo_error) => {
                error!("mongo error:{:?}", mongo_error);
                (
                    StatusCode::OK,
                    HttpResult::<String>::error("查询失败".into()),
                )
            }
            AppError::ParseError(parse_error) => {
                error!("时间转换失败:{:?}", parse_error);
                (
                    StatusCode::OK,
                    HttpResult::<String>::error("参数错误".into()),
                )
            }
            AppError::AxumError(axum_error) => {
                error!("axum error:{:?}", axum_error);
                (
                    StatusCode::OK,
                    HttpResult::<String>::error("参数错误".into()),
                )
            }
            AppError::JsonDeserializeError(json_error) => {
                error!("json error:{:?}", json_error);
                (
                    StatusCode::OK,
                    HttpResult::<String>::error("参数错误".into()),
                )
            }
        };
        let mut response = (status, serde_json::to_string(&body).unwrap()).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        response
    }
}
