use axum::{
    extract::rejection::JsonRejection,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderName, Method, StatusCode,
    },
    response::{IntoResponse, Response},
};

use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use lazy_static::lazy_static;
use serde_json::{json, Value};

#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        let json = ::serde_json::json!({ "status": status.to_string() });

        match status.as_u16() {
            200..=399 => Ok($crate::http::HttpResponse::Custom(status.as_u16(), json)),
            _ => Err($crate::http::HttpResponse::Custom(status.as_u16(),  json)),
        }

    }};
    ($status:expr, $json:expr) => {{
        let status = ::axum::http::StatusCode::from_u16($status);

        let Ok(status) = status else {
            panic!("Invalid status code: {}", $status);
        };

        if status.is_client_error() || status.is_server_error() {
            Err($crate::http::HttpResponse::Custom(status.as_u16(), $json))
        } else {
            Ok($crate::http::HttpResponse::Custom(status.as_u16(), $json))
        }
    }};
}

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

pub mod codes {
    pub const OK: u16 = 200;
    pub const CREATED: u16 = 201;
    pub const BAD_REQUEST: u16 = 400;
    pub const UNAUTHORIZED: u16 = 401;
    pub const FORBIDDEN: u16 = 403;
    pub const NOT_FOUND: u16 = 404;
    pub const CONFLICT: u16 = 409;
    pub const INTERNAL_SERVER_ERROR: u16 = 500;
}

lazy_static! {
    pub static ref ALLOWED_HTTP_HEADERS: Vec<HeaderName> =
        vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];
    pub static ref ALLOWED_HTTP_METHODS: Vec<Method> = vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ];
}
