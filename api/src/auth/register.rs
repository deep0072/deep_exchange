use actix_web::{HttpResponse, Responder, web};
use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};

use crate::{
    AppState,
    middleware::{auth_middleware::token_response, jwt_module::encode_jwt},
    models::Users,
};

pub async fn sign_up(data: web::Data<AppState>, json: web::Json<Users::Users>) -> impl Responder {
    let hash_pass = match hash(&json.pass, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            return HttpResponse::InternalServerError().json(format!("failed to hash pass {}", e));
        }
    };

    match sqlx::query("INSERT INTO users (username,email,pass) VLAUES 1$, $2, $3")
        .bind(&json.username)
        .bind(&json.email)
        .bind(&json.pass)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("user created successfully"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to register the user {}", e))
        }
    }
}
pub async fn user_login(
    data: web::Data<AppState>,
    json: web::Json<Users::Users>,
) -> impl Responder {
    match sqlx::query!(
        "SELECT username, password ,id from users where username=($1)",
        &json.username
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => match verify(&json.pass, &user.password) {
            Ok(true) => {
                let token = encode_jwt(user.username, user.id);
                let response = token_response {
                    access_token: token,
                };
                HttpResponse::Ok().json(&response)
            }
            Ok(false) => HttpResponse::Unauthorized().json("is wrong"),
            Err(BcryptError) => HttpResponse::Unauthorized().json(format!(
                "
                password is wrong{}",
                BcryptError::InvalidHash(String::from("not good"))
            )),
        },
        Err(e) => HttpResponse::NotFound().json(format!("bad request {}", e)),
    }
}
