use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use jsonwebtoken::errors::Error as JwtError;
use prisma_client_rust::QueryError;
use serde_json::{json, Value};

#[macro_export]
macro_rules! response {
    ($status:expr) => {{
        let status = $status;
        if status.is_client_error() || status.is_server_error() {
            Err($crate::http::HttpResponse::Custom(status.as_u16(), json!({"status": status.to_string()})))
        } else {
            Ok($crate::http::HttpResponse::Custom(status.as_u16(), json!({ "status": status.to_string() })))
        }
    }};
    ($status:expr, $json:expr) => {{
        let status = $status;
        if status.is_client_error() || status.is_server_error() {
            Err($crate::http::HttpResponse::Custom(status.as_u16(), $json))
        } else {
            Ok($crate::http::HttpResponse::Custom(status.as_u16(), $json))
        }
    }};
}

pub type AxumResponse = Result<HttpResponse, HttpResponse>;

#[allow(non_camel_case_types)]
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

impl From<QueryError> for HttpResponse {
    fn from(error: QueryError) -> HttpResponse {
        match error {
            _ => HttpResponse::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<JwtError> for HttpResponse {
    fn from(_: JwtError) -> HttpResponse {
        HttpResponse::UNAUTHORIZED
    }
}

pub mod routing {

    #[macro_export]
    macro_rules! get {
        ($controller:ident, $method:ident) => {
            axum::routing::get(|req| async move { $controller.$method(req).await })
        };
    }

    #[macro_export]
    macro_rules! post {
        ($controller:ident, $method:ident) => {
            axum::routing::post(move || async move { $controller.$method().await })
        };
    }

    #[macro_export]
    macro_rules! put {
        ($controller:ident, $method:ident) => {
            axum::routing::put(|req| async move { $controller.$method(req).await })
        };
    }

    #[macro_export]
    macro_rules! delete {
        ($controller:ident, $method:ident) => {
            axum::routing::delete(|req| async move { $controller.$method(req).await })
        };
    }

    #[macro_export]
    macro_rules! patch {
        ($controller:ident, $method:ident) => {
            axum::routing::patch(|req| async move { $controller.$method(req).await })
        };
    }
}
