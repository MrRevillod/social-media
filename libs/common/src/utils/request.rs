pub mod headers {

    use axum::http::HeaderMap;

    pub fn extract_header(header: &'static str, headers: &HeaderMap) -> Option<String> {
        let Some(header_value) = headers.get(header) else {
            return Option::None;
        };

        let Ok(value) = header_value.to_str() else {
            return Option::None;
        };

        Some(value.to_string())
    }
}

pub mod validations {

    use axum::{
        extract::{FromRequest, Request},
        Json,
    };

    use serde_json::json;
    use validator::{Validate, ValidationErrors};

    use crate::{
        response,
        utils::{http::codes::BAD_REQUEST, response::HttpResponse},
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

            data.validate()?;

            Ok(JsonValidator(data))
        }
    }

    impl From<ValidationErrors> for HttpResponse {
        fn from(errors: ValidationErrors) -> Self {
            HttpResponse::BadRequest(json!({ "error": errors }))
        }
    }
}
