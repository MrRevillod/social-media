use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    HeaderName, Method,
};

use lazy_static::lazy_static;

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
    pub static ref ALLOWED_HTTP_HEADERS: Vec<HeaderName> = vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];
    pub static ref ALLOWED_HTTP_METHODS: Vec<Method> = vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ];
}
