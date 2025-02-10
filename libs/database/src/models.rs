use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    id: String,
    user_id: String,
    token: String,
}
