use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use tower_cookies::{
    cookie::{time::OffsetDateTime, SameSite},
    Cookie, Cookies,
};

use chrono::{Duration as ChronoDuration, Utc};
use uuid::Uuid;

use crate::{
    constants::{ACCESS_SESSION_EXP, COOKIE_DOMAIN, ENVIRONMENT, REFRESH_SESSION_EXP},
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
    pub access: i64,
    pub refresh: i64,
}

pub struct AuthService {}

impl AuthService {
    pub async fn authenticate(
        cookies: Cookies,
        State(ctx): State<AppStateRef>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, HttpResponse> {
        let Some(cookie) = cookies.get("ACCESS") else {
            return Err(HttpResponse::UNAUTHORIZED);
        };

        let claims = jwt::verify(&cookie.value().to_string(), None)?;

        let user_id = Uuid::parse_str(&claims.user_id)?;
        let session_id = Uuid::parse_str(&claims.session_id)?;

        let session = SessionRepository::find_by_id(&ctx.prisma, session_id).await?;

        if session.is_none() || !session.as_ref().unwrap().active {
            return Err(HttpResponse::UNAUTHORIZED);
        }

        let Some(user) = UserRepository::find_by_id(&ctx.prisma, user_id).await? else {
            return Err(HttpResponse::UNAUTHORIZED);
        };

        req.extensions_mut().insert::<User>(user);
        req.extensions_mut().insert::<Claims>(claims);
        req.extensions_mut().insert::<Uuid>(session_id);

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

    pub fn add_session_cookies(cookies: &mut Cookies, tokens: Vec<String>, exps: ExpirationTimes) {
        let mut access = Cookie::new("ACCESS", tokens[0].clone());
        let exp = OffsetDateTime::from_unix_timestamp(exps.access).unwrap();

        access.set_http_only(true);
        access.set_path("/");
        access.set_expires(exp);
        access.set_same_site(Some(SameSite::Lax));
        access.set_secure(*ENVIRONMENT == "production");
        access.set_domain(COOKIE_DOMAIN.as_str());

        let mut refresh = Cookie::new("REFRESH", tokens[1].clone());
        let exp = OffsetDateTime::from_unix_timestamp(exps.refresh).unwrap();

        refresh.set_http_only(true);
        refresh.set_path("/");
        refresh.set_expires(exp);
        refresh.set_same_site(Some(SameSite::Lax));
        refresh.set_secure(*ENVIRONMENT == "production");
        refresh.set_domain(COOKIE_DOMAIN.as_str());

        cookies.add(access);
        cookies.add(refresh);
    }

    pub fn remove_session_cookies(cookies: &Cookies) {
        let mut access = Cookie::new("ACCESS", "");

        access.set_http_only(true);
        access.set_path("/");
        access.set_expires(OffsetDateTime::UNIX_EPOCH);
        access.set_same_site(Some(SameSite::Lax));
        access.set_secure(*ENVIRONMENT == "production");

        access.set_domain(COOKIE_DOMAIN.as_str());

        let mut refresh = Cookie::new("REFRESH", "");

        refresh.set_http_only(true);
        refresh.set_path("/");
        refresh.set_expires(OffsetDateTime::UNIX_EPOCH);
        refresh.set_same_site(Some(SameSite::Lax));
        refresh.set_secure(*ENVIRONMENT == "production");
        refresh.set_domain(COOKIE_DOMAIN.as_str());

        cookies.remove(access);
        cookies.remove(refresh);

        dbg!(&cookies);
    }
}
