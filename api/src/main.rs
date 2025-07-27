use actix_web::{App, HttpServer, web};
use db::conn::Pool;
use dotenvy::dotenv;
use routes::urls::config;
use std::env;

mod auth;
mod db;
mod middleware;
mod models;
mod redis_manager;
mod routes;

struct AppState {
    db: Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    dotenv().ok();

    let pg_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => "could not interepret {e}".to_string(),
    };

    let pool = db::conn::init__pool(&pg_url).await;

    HttpServer::new(move || {
        App::new()
            //  .app_Data() shove the database data from top to bottom
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
