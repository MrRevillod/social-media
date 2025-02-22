use axum::{extract::State, http::HeaderMap, Extension};

use serde_json::json;
use tower_cookies::Cookies;
use uuid::Uuid;

use common::{
    http::{
        codes::{OK, UNAUTHORIZED},
        AxumResponse,
    },
    repositories::{session::SessionRepository, user::UserRepository},
    response,
    services::{
        auth::AuthService,
        jwt::{self, Claims},
        state::AppStateRef,
    },
    utils::body::JsonValidator,
};

use super::schemas::*;

pub async fn login(
    State(ctx): State<AppStateRef>,
    mut cookies: Cookies,
    headers: HeaderMap,
    JsonValidator(body): JsonValidator<LoginRequest>,
) -> AxumResponse {
    let user = UserRepository::find_one(&ctx.prisma, None, Some(&body.email)).await?;

    let Some(user) = user else {
        return response!(UNAUTHORIZED, json!({ "message": "Invalid credentials" }));
    };

    if !user.validated {
        return response!(401, json!({ "message": "The account is not validated" }));
    }

    // Compare the req password with the db hashed password
    if !bcrypt::verify(&body.password, &user.password)? {
        return response!(UNAUTHORIZED, json!({ "message": "Invalid credentials" }));
    }

    let user_id = user.id.to_string();
    let session_id = Uuid::new_v4().to_string();
    let session_exps = AuthService::get_exp_times();

    let access_payload = Claims::new(user_id.clone(), session_id.clone(), session_exps.access);
    let refresh_payload = Claims::new(user_id, session_id, session_exps.refresh);

    let tokens = vec![
        jwt::sign(access_payload, None)?,
        jwt::sign(refresh_payload, None)?,
    ];

    let client_ip_address = headers
        .get("X-Real-IP")
        .and_then(|ip| ip.to_str().ok())
        .map(String::from);

    let client_user_agent = headers
        .get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .map(String::from);

    SessionRepository::create(
        &ctx.prisma,
        tokens[1].clone(),
        user.id,
        client_ip_address.clone(),
        client_user_agent,
        session_exps.refresh,
    )
    .await?;

    AuthService::add_session_cookies(&mut cookies, tokens, session_exps);

    response!(OK)
}

pub async fn logout(
    State(ctx): State<AppStateRef>,
    Extension(payload): Extension<Claims>,
    mut cookies: Cookies,
) -> AxumResponse {
    SessionRepository::desactivate(&ctx.prisma, payload.session_id).await?;
    AuthService::remove_session_cookies(&mut cookies);

    response!(OK, json!({ "message": "Logged out" }))
}
