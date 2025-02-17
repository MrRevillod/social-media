use axum::{extract::State, http::StatusCode};

use common::{
    http::AxumResponse, prisma::user, repositories::UserRepository, response,
    services::state::AppStateRef, utils::body::JsonValidator,
};

use super::schemas::LoginRequest;
use serde_json::json;

pub async fn login(
    State(ctx): State<AppStateRef>,
    JsonValidator(body): JsonValidator<LoginRequest>,
) -> AxumResponse {
    let user = UserRepository::find_one(&ctx.prisma, user::email::equals(body.email)).await?;

    if user.is_none() {
        return response!(StatusCode::NOT_FOUND);
    }

    response!(StatusCode::OK, json!({ "user": user }))
}
