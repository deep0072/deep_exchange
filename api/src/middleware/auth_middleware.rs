use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    http::{Error, header::AUTHORIZATION},
    middleware::{self, Next},
};
use serde::Serialize;

use crate::middleware::jwt_module;

use super::jwt_module::decode_jwt;

pub struct UserClaims {
    exp: usize,
    iat: usize,
    pub user: String,
    pub id: u32,
}

#[derive(Serialize)]
pub struct token_response {
    pub access_token: String,
}

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| ErrorUnauthorized("Missing auth header"))?;
    let token = auth_header
        .to_str()
        .map_err(|_| ErrorUnauthorized("invalid header"))?
        .strip_prefix("Bearer ")
        .ok_or(ErrorUnauthorized("Invalid scheme"))?;

    let claim: jsonwebtoken::TokenData<jwt_module::UserClaim> =
        decode_jwt(token.trim().to_owned()).map_err(|_| ErrorUnauthorized("invalid token"))?;
    req.extensions_mut().insert(claim.claims);
    let res = next.call(req).await?;
    Ok(res)
}
