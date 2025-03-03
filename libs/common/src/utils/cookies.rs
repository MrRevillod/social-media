use std::fmt::Display;

use tower_cookies::{
    cookie::{time::OffsetDateTime, SameSite},
    Cookie, Cookies,
};

use crate::constants::{COOKIE_DOMAIN, ENVIRONMENT};

#[derive(Debug, Clone)]
pub struct ServerCookie {}

impl ServerCookie {
    pub fn new<S>(name: S, value: S, exp: i64) -> Cookie<'static>
    where
        S: Into<String> + Display,
    {
        let mut cookie = Cookie::new(name.to_string(), value.to_string());

        cookie.set_http_only(true);
        cookie.set_secure(*ENVIRONMENT == "production");

        cookie.set_domain(COOKIE_DOMAIN.to_owned());
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);

        cookie.set_expires(OffsetDateTime::from_unix_timestamp(exp).unwrap());

        cookie
    }
}

pub fn add_cookie(cookie: Cookie<'static>, cookies_ref: &Cookies) {
    cookies_ref.add(cookie);
}

pub fn add_cookies(cookies: Vec<Cookie<'static>>, cookies_ref: &Cookies) {
    for cookie in cookies {
        cookies_ref.add(cookie);
    }
}

pub fn remove_cookie(name: &'static str, cookies_ref: &Cookies) {
    cookies_ref.remove(Cookie::new(name, ""));
}

pub fn remove_cookies(names: Vec<&'static str>, cookies_ref: &Cookies) {
    for name in names.iter() {
        cookies_ref.remove(Cookie::new(*name, ""));
    }
}
