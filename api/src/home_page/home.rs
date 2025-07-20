use actix_web::{HttpResponse, Responder};

pub async fn place_order() -> impl Responder {
    HttpResponse::Ok().json("welcome to page")
}
