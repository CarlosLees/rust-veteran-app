use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResult<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> HttpResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            message: String::from("成功"),
            data: Some(data),
        }
    }

    pub fn ok_with_message() -> Self {
        Self {
            code: 200,
            message: String::from("成功"),
            data: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            code: 555,
            message,
            data: None,
        }
    }
}

impl<T> IntoResponse for HttpResult<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        let mut response = (StatusCode::OK, serde_json::to_string(&self).unwrap()).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        response
    }
}
