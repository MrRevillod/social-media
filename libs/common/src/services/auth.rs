use std::{str::FromStr, time::Duration};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use chrono::{Duration as ChronoDuration, Utc};
use cookie_rs::prelude::{Cookie, SameSite};
use tower_cookies::{Cookie as TowerCookie, Cookies};

use crate::{
    constants::{ACCESS_SESSION_EXP, REFRESH_SESSION_EXP},
    http::HttpResponse,
    models::User,
    repositories::{session::SessionRepository, user::UserRepository},
};

use super::{
    jwt::{self, Claims},
    state::AppStateRef,
};

#[derive(Debug)]
pub struct ExpirationTimes {
    pub access: u64,
    pub refresh: u64,
}

pub struct AuthService {}

impl AuthService {
    pub async fn authenticate(
        cookies: Cookies,
        State(ctx): State<AppStateRef>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, HttpResponse> {
        let Some(cookie) = cookies.get("access") else {
            return Err(HttpResponse::UNAUTHORIZED);
        };

        let token = cookie.value().to_string();
        let payload = jwt::verify(&token, None)?;

        let session = SessionRepository::find_one(&ctx.prisma, &payload.session_id).await?;

        if session.is_none() || !session.unwrap().active {
            return Err(HttpResponse::UNAUTHORIZED);
        }

        let user = UserRepository::find_by_id(&ctx.prisma, &payload.user_id).await?;

        if user.is_none() {
            return Err(HttpResponse::UNAUTHORIZED);
        }

        req.extensions_mut().insert::<User>(user.unwrap());
        req.extensions_mut().insert::<Claims>(payload);

        Ok(next.run(req).await)
    }

    pub fn get_exp_times() -> ExpirationTimes {
        let base = Utc::now().timestamp() as u64;
        let access_exp = ChronoDuration::minutes(*ACCESS_SESSION_EXP).num_seconds() as u64;
        let refresh_exp = ChronoDuration::days(*REFRESH_SESSION_EXP).num_seconds() as u64;

        ExpirationTimes {
            access: base + access_exp,
            refresh: base + refresh_exp,
        }
    }

    pub fn add_session_cookies(cookies: &mut Cookies, tokens: Vec<String>, exps: ExpirationTimes) {
        let access = Cookie::builder("access", tokens[0].clone())
            .http_only(true)
            .max_age(Duration::from_secs(exps.access))
            .same_site(SameSite::Lax)
            .build();

        let refresh = Cookie::builder("refresh", tokens[1].clone())
            .http_only(true)
            .max_age(Duration::from_secs(exps.refresh))
            .same_site(SameSite::Lax)
            .build();

        cookies.add(TowerCookie::from_str(&access.to_string()).unwrap());
        cookies.add(TowerCookie::from_str(&refresh.to_string()).unwrap());
    }

    pub fn remove_session_cookies(cookies: &mut Cookies) {
        let access = Cookie::builder("access", "")
            .http_only(true)
            .max_age(Duration::from_secs(0))
            .same_site(SameSite::Lax)
            .build();

        let refresh = Cookie::builder("refresh", "")
            .http_only(true)
            .max_age(Duration::from_secs(0))
            .same_site(SameSite::Lax)
            .build();

        cookies.add(TowerCookie::from_str(&access.to_string()).unwrap());
        cookies.add(TowerCookie::from_str(&refresh.to_string()).unwrap());
    }
}
