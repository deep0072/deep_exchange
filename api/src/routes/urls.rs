use actix_web::{
    middleware::from_fn,
    web::{self, route},
};

use crate::{
    auth::register::{sign_up, user_login},
    home_page::home::{self, home},
    middleware::auth_middleware::check_auth_middleware,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health_check", web::get().to(home))
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(user_login))
                    .route("/sign_up", web::post().to(sign_up)),
            )
            .service(
                web::scope("/dashboard")
                    .wrap(from_fn(check_auth_middleware))
                    .service(web::resource("order").route(web::get().to(home))),
            ),
    );
}
