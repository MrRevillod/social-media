pub mod database;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use std::env;

use axum::{http::HeaderValue, Router};

use crate::{
    database::PostgresClient,
    services::{logger::HttpLogger, state::AppState},
    utils::http::{ALLOWED_HTTP_HEADERS, ALLOWED_HTTP_METHODS},
};

use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub mod constants {

    use super::get_env_var;
    use lazy_static::lazy_static;

    lazy_static! {

        // Database variables

        pub static ref POSTGRES_DATABASE_URL: String = get_env_var("POSTGRES_DATABASE_URL");
        pub static ref POSTGRES_TEST_DATABASE_URL: String =
            get_env_var("POSTGRES_TEST_DATABASE_URL");

        // Session and jwt variables

        pub static ref JWT_SECRET: String = get_env_var("JWT_SECRET");
        pub static ref ACCESS_SESSION_EXP: i64 =
            get_env_var("ACCESS_SESSION_EXP").parse::<i64>().unwrap();
        pub static ref REFRESH_SESSION_EXP: i64 =
            get_env_var("REFRESH_SESSION_EXP").parse::<i64>().unwrap();

        // Server variables

        pub static ref BASE_SERVER_URL: String = get_env_var("BASE_SERVER_URL");
        pub static ref ENVIRONMENT: String = get_env_var("ENVIRONMENT");
        pub static ref COOKIE_DOMAIN: String = get_env_var("COOKIE_DOMAIN");

        // Project email variables

        pub static ref PROJECT_EMAIL_ADDRESS: String = get_env_var("PROJECT_EMAIL_ADDRESS");
        pub static ref PROJECT_EMAIL_PASSWORD: String = get_env_var("PROJECT_EMAIL_PASSWORD");
        pub static ref PROJECT_EMAIL_SMTP_SERVER: String = get_env_var("PROJECT_EMAIL_SMTP_SERVER");
        pub static ref PROJECT_EMAIL_SMTP_PORT: String = get_env_var("PROJECT_EMAIL_SMTP_PORT");

        // RabbitMQ variables

        pub static ref RABBITMQ_ADDR: String = get_env_var("RABBITMQ_ADDR");
        pub static ref RABBITMQ_EXCHANGE_NAME: String = get_env_var("RABBITMQ_EXCHANGE_NAME");
        pub static ref RABBITMQ_EMAIL_TOPIC_QUEUE: String = get_env_var("RABBITMQ_EMAIL_TOPIC_QUEUE");
        pub static ref RABBITMQ_FILE_TOPIC_QUEUE: String = get_env_var("RABBITMQ_FILE_TOPIC_QUEUE");
    }
}

pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("ENV - PANIC - {} env var not found", key))
}

pub fn check_env_vars() {
    let _ = constants::JWT_SECRET;
    let _ = constants::POSTGRES_DATABASE_URL;
    let _ = constants::POSTGRES_TEST_DATABASE_URL;
    let _ = constants::ACCESS_SESSION_EXP;
    let _ = constants::REFRESH_SESSION_EXP;
    let _ = constants::BASE_SERVER_URL;
    let _ = constants::ENVIRONMENT;
    let _ = constants::COOKIE_DOMAIN;
}

pub async fn create_app<F>(router_fns: Vec<F>) -> Router
where
    F: Fn(AppState) -> Router,
{
    check_env_vars();

    let database = PostgresClient::new(&constants::POSTGRES_DATABASE_URL).await;
    let app_state = AppState::new(database.clone());

    let http_logger = HttpLogger::new();
    let cookie_layer = CookieManagerLayer::new();

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(ALLOWED_HTTP_METHODS.to_owned())
        .allow_headers(ALLOWED_HTTP_HEADERS.to_owned())
        .allow_origin(constants::BASE_SERVER_URL.parse::<HeaderValue>().unwrap());

    let mut app = Router::new();

    for router_fn in router_fns {
        app = app.merge(router_fn(app_state.clone()));
    }

    app = app.layer(cors).layer(cookie_layer).layer(http_logger.layer);

    app
}

pub mod regex {

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
    }
}
