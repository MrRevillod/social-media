pub mod cookies;
pub mod http;
pub mod request;
pub mod response;

pub mod uuid {

    use serde_json::json;
    pub use uuid::{Error, Uuid};

    use super::response::HttpResponse;

    pub fn generate() -> Uuid {
        Uuid::new_v4()
    }

    pub fn parse_str(uuid: &str) -> Result<Uuid, Error> {
        Uuid::parse_str(uuid)
    }

    impl From<Error> for HttpResponse {
        fn from(_: Error) -> Self {
            HttpResponse::BadRequest(json!({ "error": "Invalid identifier" }))
        }
    }
}
