use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use lib_utils::HttpResult;
use thiserror::Error;

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
        };
        let mut response = (status, serde_json::to_string(&body).unwrap()).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        response
    }
}
