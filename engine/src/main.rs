use redis::{Commands, RedisResult};

mod redis_manager;
mod trade;
#[actix_web::main]
async fn main() -> RedisResult<()> {
    println!("Hello, world!");
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut client_conn = client.get_connection().unwrap();
    let response: Option<String> =
        client_conn.rpush("message".to_string(), "hi there".to_string())?;
    loop {
        let response: Option<String> = client_conn.rpop("message".to_string(), None)?;
        if let Some(msg) = response {
            println!("Popped msg {}", msg);
        } else {
            continue;
        }
    }
    Ok(())
}
