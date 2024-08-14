use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, format!("NotFound")).into_response(),
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, format!("Unauthorized")).into_response()
            }
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error"),
            )
                .into_response(),
        }
    }
}
