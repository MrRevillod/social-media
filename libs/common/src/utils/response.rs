use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use serde_json::{json, Value};

pub type AxumResponse = Result<HttpResponse, HttpResponse>;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum HttpResponse {
    OK,
    CREATED,
    BAD_REQUEST,
    CONFLICT,
    FORBIDDEN,
    INTERNAL_SERVER_ERROR,
    NOT_FOUND,
    UNAUTHORIZED,
    BadRequest(Value),
    Conflict(Value),
    Custom(u16, Value),
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> Response {
        let (code, data) = match self {
            HttpResponse::OK => (200, json!({})),
            HttpResponse::CREATED => (201, json!({})),
            HttpResponse::BAD_REQUEST => (400, json!({"error": "Bad Request"})),
            HttpResponse::CONFLICT => (409, json!({"error": "Conflict"})),
            HttpResponse::FORBIDDEN => (403, json!({"error": "Forbidden"})),
            HttpResponse::INTERNAL_SERVER_ERROR => (500, json!({"error": "Internal Server Error"})),
            HttpResponse::NOT_FOUND => (404, json!({"error": "Not Found"})),
            HttpResponse::UNAUTHORIZED => (401, json!({"error": "Unauthorized"})),
            HttpResponse::BadRequest(data) => (400, data),
            HttpResponse::Conflict(data) => (409, data),
            HttpResponse::Custom(code, data) => (code.into(), data),
        };

        (StatusCode::from_u16(code).unwrap(), axum::Json(data)).into_response()
    }
}

impl From<JwtError> for HttpResponse {
    fn from(_: JwtError) -> HttpResponse {
        HttpResponse::UNAUTHORIZED
    }
}

impl From<sqlx::Error> for HttpResponse {
    fn from(e: sqlx::Error) -> HttpResponse {
        dbg!(&e);

        HttpResponse::INTERNAL_SERVER_ERROR
    }
}

impl From<JsonRejection> for HttpResponse {
    fn from(_: JsonRejection) -> HttpResponse {
        HttpResponse::BAD_REQUEST
    }
}

impl From<BcryptError> for HttpResponse {
    fn from(_: BcryptError) -> HttpResponse {
        HttpResponse::INTERNAL_SERVER_ERROR
    }
}

#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ "status": status.to_string() });

        match status.as_u16() {
            200..=399 => Ok($crate::utils::response::HttpResponse::Custom(status.as_u16(), json)),
            _ => Err($crate::utils::response::HttpResponse::Custom(status.as_u16(),  json)),
        }

    }};
    ($status:expr, $json:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        if status.is_client_error() || status.is_server_error() {
            Err($crate::utils::response::HttpResponse::Custom(status.as_u16(), $json))
        } else {
            Ok($crate::utils::response::HttpResponse::Custom(status.as_u16(), $json))
        }
    }};
}
