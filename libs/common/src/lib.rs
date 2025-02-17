pub mod database;
pub mod http;
#[allow(clippy::module_inception, warnings, clippy::err_expect)]
pub mod prisma;
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
    }
}

pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} env var not found", key))
}
