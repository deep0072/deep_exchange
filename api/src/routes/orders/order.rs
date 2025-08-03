use crate::middleware::jwt_module::UserClaim;
use crate::redis_manager::RedisManager;
use crate::routes::constant::OrderPayload;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, error::ErrorUnauthorized, web};

pub async fn place_order(mut order: web::Json<OrderPayload>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // Then get the user claims from the extensions
    let user_claims = extensions.get::<UserClaim>().unwrap();
    order.user_id = Some(user_claims.id.clone());
    println!("{:?}", order);

    // then call redis instance
    match RedisManager::get_instance().await {
        Ok(redis_manager) => match redis_manager.send_and_await(order.into_inner()).await {
            Ok(_) => HttpResponse::Ok().json("Order placed successfully"),
            Err(err) => {
                HttpResponse::InternalServerError().json(format!(" failed to place order: {}", err))
            }
        },
        Err(err) => {
            // Handle error
            HttpResponse::InternalServerError().json(format!(" failed to place order: {}", err))
        }
    }
}
