use axum::extract::State;
use common::http::codes::{CONFLICT, CREATED};
use serde_json::json;

use common::{
    http::AxumResponse, repositories::user::UserRepository, response, services::state::AppStateRef,
    utils::body::JsonValidator,
};

use super::schemas::RegisterRequest;

pub async fn create(
    State(ctx): State<AppStateRef>,
    JsonValidator(body): JsonValidator<RegisterRequest>,
) -> AxumResponse {
    let user = UserRepository::find_one(&ctx.prisma, None, Some(&body.email)).await?;

    if user.is_some() {
        return response!(CONFLICT, json!({ "conflicts": vec!["email"] }));
    }

    dbg!(&body);
    dbg!(&user);

    UserRepository::create(
        &ctx.prisma,
        body.username.clone(),
        body.email.clone(),
        bcrypt::hash(&body.password, 10)?,
    )
    .await?;

    response!(CREATED)
}
