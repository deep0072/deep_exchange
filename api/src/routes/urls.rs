use actix_web::{
    middleware::from_fn,
    web::{self, route},
};

use crate::{
    auth::register::{sign_up, user_login},
    middleware::auth_middleware::check_auth_middleware,
    routes::orders::order::place_order,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(user_login))
                    .route("/sign_up", web::post().to(sign_up)),
            )
            .service(
                web::scope("/exchange")
                    .wrap(from_fn(check_auth_middleware))
                    .service(web::resource("/order").route(web::post().to(place_order))),
            ),
    );
}
