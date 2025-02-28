pub mod body;

pub mod uuid {

    use serde_json::json;
    use uuid::{Error, Uuid};

    use crate::http::HttpResponse;

    pub fn generate() -> Uuid {
        Uuid::new_v4()
    }

    pub fn parse(uuid: &str) -> Result<Uuid, Error> {
        Uuid::parse_str(uuid)
    }

    impl From<Error> for HttpResponse {
        fn from(_: Error) -> Self {
            HttpResponse::BadRequest(json!({ "error": "Invalid identifier" }))
        }
    }
}
