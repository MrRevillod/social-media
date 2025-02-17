use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// let (mut parts, body) = req.into_parts();
// let req = Request::from_parts(parts, body);

use serde_json::json;
use validator::Validate;

pub struct JsonValidator<T>(pub T);

impl<S, T> FromRequest<S> for JsonValidator<T>
where
    T: Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid JSON body" })),
            )
                .into_response()
        })?;

        data.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response()
        })?;

        Ok(JsonValidator(data))
    }
}
