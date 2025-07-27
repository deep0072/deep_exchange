use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Users {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}
