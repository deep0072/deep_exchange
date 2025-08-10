use crate::middleware::jwt_module::UserClaim;
use crate::redis_manager::RedisManager;
use crate::routes::types::{CreateOrderData, MessageToEngine};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, error::ErrorUnauthorized, web};

pub async fn place_order(
    mut order: web::Json<CreateOrderData>,
    req: HttpRequest,
) -> impl Responder {
    let extensions = req.extensions();
    // Then get the user claims from the extensions
    let user_claims = extensions.get::<UserClaim>().unwrap();
    order.user_id = Some(user_claims.id.clone());

    let message_to_engine: MessageToEngine = MessageToEngine::CreateOrder(order.into_inner());

    // then call redis instance
    match RedisManager::get_instance().await {
        Ok(redis_manager) => match redis_manager.send_and_await(message_to_engine).await {
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
