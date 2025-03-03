use axum::extract::State;
use serde_json::json;

use common::utils::http::codes::{CONFLICT, CREATED};

use common::{
    repositories::user::UserRepository,
    response,
    services::state::AppState,
    utils::{request::validations::JsonValidator, response::AxumResponse},
};

use super::schemas::RegisterRequest;

pub async fn create(
    State(ctx): State<AppState>,
    JsonValidator(body): JsonValidator<RegisterRequest>,
) -> AxumResponse {
    let user = UserRepository::find_one(&ctx.prisma, None, Some(&body.email)).await?;

    if user.is_some() {
        return response!(CONFLICT, json!({ "conflicts": vec!["email"] }));
    }

    UserRepository::create(
        &ctx.prisma,
        body.username.clone(),
        body.email.clone(),
        bcrypt::hash(&body.password, 10)?,
    )
    .await?;

    response!(CREATED)
}
