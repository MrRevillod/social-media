use axum::{
    extract::{FromRequest, Request},
    Json,
};

// let (mut parts, body) = req.into_parts();
// let req = Request::from_parts(parts, body);

use serde_json::json;
use validator::{Validate, ValidationErrors};

use crate::{
    http::{codes::BAD_REQUEST, HttpResponse},
    response,
};

pub struct JsonValidator<T>(pub T);

impl<S, T> FromRequest<S> for JsonValidator<T>
where
    T: Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = HttpResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, HttpResponse> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|_| {
            response!(BAD_REQUEST, json!({ "error": "Invalid JSON body" })).unwrap_err()
        })?;

        println!("Validating...");

        data.validate()?;

        println!("Validated...");

        Ok(JsonValidator(data))
    }
}

impl From<ValidationErrors> for HttpResponse {
    fn from(errors: ValidationErrors) -> Self {
        HttpResponse::BadRequest(json!({ "error": errors }))
    }
}
