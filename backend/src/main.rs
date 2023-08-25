pub mod api;
pub mod model;

use api::ws::chat_route;
// use model::get_language_model;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(chat_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
