pub mod api;
pub mod handler;

use actix_web::{App, HttpServer};
use api::ws::chat_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(chat_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
