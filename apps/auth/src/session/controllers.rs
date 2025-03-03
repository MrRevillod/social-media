use axum::{extract::State, http::HeaderMap, Extension};

use serde_json::json;
use tower_cookies::Cookies;
use uuid::Uuid;

use common::{
    repositories::{session::SessionRepository, user::UserRepository},
    response,
    services::{
        auth::AuthService,
        jwt::{self, Claims},
        state::AppState,
    },
    utils::{
        http::codes::{OK, UNAUTHORIZED},
        request::{headers::extract_header, validations::JsonValidator},
        response::AxumResponse,
        uuid,
    },
};

use super::schemas::*;

pub async fn login(
    State(ctx): State<AppState>,
    headers: HeaderMap,
    cookies: Cookies,
    JsonValidator(body): JsonValidator<LoginRequest>,
) -> AxumResponse {
    let user = UserRepository::find_one(&ctx.prisma, None, Some(&body.email)).await?;

    let Some(user) = user else {
        return response!(UNAUTHORIZED, json!({ "message": "Invalid credentials" }));
    };

    if !user.validated {
        return response!(
            UNAUTHORIZED,
            json!({ "message": "The account is not validated" })
        );
    }

    // Compare the req password with the db hashed password
    if !bcrypt::verify(&body.password, &user.password)? {
        return response!(UNAUTHORIZED, json!({ "message": "Invalid credentials" }));
    }

    let user_id = user.id.to_string();
    let session_id = Uuid::new_v4().to_string();
    let session_exps = AuthService::get_exp_times();

    let access_payload = Claims::new(user_id.clone(), session_id.clone(), session_exps.access);
    let refresh_payload = Claims::new(user_id, session_id.clone(), session_exps.refresh);

    let tokens = vec![
        jwt::sign(access_payload, None)?,
        jwt::sign(refresh_payload, None)?,
    ];

    let Some(req_origin) = extract_header("X-Real-IP", &headers) else {
        return response!(
            UNAUTHORIZED,
            json!({ "message": "Fail getting the request origin" })
        );
    };

    let req_user_agent = extract_header("User-Agent", &headers);

    SessionRepository::create(
        &ctx.prisma,
        &session_id,
        tokens[1].clone(),
        user.id,
        Some(req_origin),
        req_user_agent,
        session_exps.refresh,
    )
    .await?;

    AuthService::add_session_cookies(&cookies, tokens, session_exps);

    response!(OK)
}

pub async fn logout(
    State(ctx): State<AppState>,
    Extension(session_id): Extension<Uuid>,
    cookies: Cookies,
) -> AxumResponse {
    SessionRepository::desactivate(&ctx.prisma, session_id).await?;
    AuthService::remove_session_cookies(&cookies);

    response!(OK, json!({ "message": "The session has ended" }))
}

pub async fn validate_session() -> AxumResponse {
    response!(OK, json!({ "message": "The session is valid" }))
}

pub async fn refresh(State(ctx): State<AppState>, cookies: Cookies) -> AxumResponse {
    let Some(cookie) = cookies.get("REFRESH") else {
        return response!(UNAUTHORIZED);
    };

    // Get the claims from the refresh token

    let claims = jwt::verify(&cookie.value().to_string(), None)?;

    let user_id = uuid::parse_str(&claims.user_id)?;
    let session_id = uuid::parse_str(&claims.session_id)?;

    // if the user or the session doesn't exist, return unauthorized

    if let None = UserRepository::find_by_id(&ctx.prisma, user_id).await? {
        return response!(UNAUTHORIZED);
    };

    if let None = SessionRepository::find_by_id(&ctx.prisma, session_id).await? {
        AuthService::remove_session_cookies(&cookies);
        return response!(UNAUTHORIZED);
    };

    // Build a new access token and refresh the session

    let session_exp = AuthService::get_exp_times().access;
    let new_claims = Claims::new(claims.user_id, claims.session_id, session_exp);

    let new_token = jwt::sign(new_claims, None)?;

    AuthService::refresh_session(&cookies, new_token, session_exp);

    response!(OK, json!({ "message": "The session has been refreshed" }))
}
