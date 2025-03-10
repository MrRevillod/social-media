use std::ops::Index;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use chrono::{Duration as ChronoDuration, Utc};
use tower_cookies::Cookies;

use crate::{
    constants::{ACCESS_SESSION_EXP, REFRESH_SESSION_EXP},
    models::User,
    repositories::{session::SessionRepository, user::UserRepository},
    utils::{cookies::ServerCookie, response::HttpResponse, uuid},
};

use super::{
    jwt::{self, Claims, Secret},
    state::AppState,
};

#[derive(Debug)]
pub struct ExpirationTimes {
    pub access: i64,
    pub refresh: i64,
}

pub struct AuthService {}

impl AuthService {

    pub async fn authenticate(
        State(ctx): State<AppState>,
        cookies: Cookies,
        mut req: Request,
        next: Next,
    ) -> Result<Response, HttpResponse> {
        let Some(cookie) = cookies.get("ACCESS") else {
            return Err(HttpResponse::UNAUTHORIZED);
        };

        let claims = jwt::verify(&cookie.value().to_string(), Secret::Default)?;

        let user_id = uuid::parse_str(&claims.user_id)?;
        let session_id = uuid::parse_str(&claims.session_id)?;

        let session = SessionRepository::find_by_id(&ctx.prisma, session_id).await?;

        if session.is_none() || !session.as_ref().unwrap().active {
            return Err(HttpResponse::UNAUTHORIZED);
        }

        let Some(user) = UserRepository::find_by_id(&ctx.prisma, user_id).await? else {
            return Err(HttpResponse::UNAUTHORIZED);
        };

        req.extensions_mut().insert::<User>(user);
        req.extensions_mut().insert::<Claims>(claims);
        req.extensions_mut().insert::<uuid::Uuid>(session_id);

        Ok(next.run(req).await)
    }

    pub fn get_exp_times() -> ExpirationTimes {
        let base = Utc::now().timestamp();
        let access_exp = ChronoDuration::minutes(*ACCESS_SESSION_EXP).num_seconds();
        let refresh_exp = ChronoDuration::days(*REFRESH_SESSION_EXP).num_seconds();

        ExpirationTimes {
            access: base + access_exp,
            refresh: base + refresh_exp,
        }
    }

    pub fn add_session_cookies(cookies: &Cookies, tokens: Vec<String>, exps: ExpirationTimes) {
        cookies.add(ServerCookie::new("ACCESS", &tokens.index(0), exps.access));
        cookies.add(ServerCookie::new("REFRESH", &tokens.index(1), exps.refresh));
    }

    pub fn remove_session_cookies(cookies: &Cookies) {
        cookies.remove(ServerCookie::new("ACCESS", "", 0));
        cookies.remove(ServerCookie::new("REFRESH", "", 0));
    }

    pub fn refresh_session(cookies: &Cookies, token: String, exp: i64) {
        cookies.add(ServerCookie::new("ACCESS", &token, exp));
    }
}
