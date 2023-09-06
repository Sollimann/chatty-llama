pub mod api;
pub mod handler;

use actix_web::{middleware::Logger, web, App, HttpServer};
use handler::model::load_language_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::api::ws::ws;
    use std::sync::Arc;
    env_logger::init();

    let llama = load_language_model();
    println!("model loaded");

    // use the model to generate text from a prompt
    let llama = Arc::new(llama);
    let model = web::Data::new(llama);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(model.clone())
            .route("/ws/", web::get().to(ws))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
