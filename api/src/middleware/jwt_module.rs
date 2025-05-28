use actix_web::guard::Header;
use chrono::{self, Duration};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaim {
    exp: usize,
    iat: usize,
    pub user: String,
    pub id: i32,
}

pub fn encode_jwt(username: String, id: i32) -> String {
    let secret_key: String = match env::var("JWT_KEY") {
        Ok(val) => val,
        Err(e) => "unable to find secret key".to_string(),
    };

    let now = chrono::Utc::now();
    let expire = Duration::hours(24);
    let user_claim = UserClaim {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        user: username,
        id,
    };

    let token = match encode(
        &Header::default(),
        &user_claim,
        &EncodingKey::from_secret(secret_key.as_ref()),
    ) {
        Ok(val) => val,
        Err(e) => "encoding err {e}".to_string(),
    };
    token
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<UserClaim>, jsonwebtoken::errors::Error> {
    let secret_key: String = match env::var("JWT_KEY") {
        Ok(val) => val,
        Err(e) => "unable to find secret key".to_string(),
    };

    let claim: Result<TokenData<UserClaim>, jsonwebtoken::errors::Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    );
    claim
}
