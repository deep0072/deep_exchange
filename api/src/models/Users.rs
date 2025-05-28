use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Users {
    pub username: String,
    pub email: Option<String>,
    pub pass: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}
