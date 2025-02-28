pub mod database;
pub mod http;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use std::env;

pub mod constants {

    use super::get_env_var;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref JWT_SECRET: String = get_env_var("JWT_SECRET");
        pub static ref POSTGRES_DATABASE_URL: String = get_env_var("POSTGRES_DATABASE_URL");
        pub static ref ACCESS_SESSION_EXP: i64 =
            get_env_var("ACCESS_SESSION_EXP").parse::<i64>().unwrap();
        pub static ref REFRESH_SESSION_EXP: i64 =
            get_env_var("REFRESH_SESSION_EXP").parse::<i64>().unwrap();
        pub static ref BASE_SERVER_URL: String = get_env_var("BASE_SERVER_URL");
        pub static ref ENVIRONMENT: String = get_env_var("ENVIRONMENT");
        pub static ref COOKIE_DOMAIN: String = get_env_var("COOKIE_DOMAIN");
    }
}

pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("ENV - PANIC - {} env var not found", key))
}

pub fn check_env_vars() -> () {
    let _ = constants::JWT_SECRET;
    let _ = constants::POSTGRES_DATABASE_URL;
    let _ = constants::ACCESS_SESSION_EXP;
    let _ = constants::REFRESH_SESSION_EXP;
    let _ = constants::BASE_SERVER_URL;
    let _ = constants::ENVIRONMENT;
    let _ = constants::COOKIE_DOMAIN;
}

pub mod regex {

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
    }
}
