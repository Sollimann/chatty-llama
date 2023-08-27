pub mod api;
pub mod handler;

use std::io::Write;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::ws::chat_route;
use handler::model::{inference_callback, load_language_model, session_setup};

fn print_token(t: String) {
    print!("{t}");
    let mut stdout = std::io::stdout().lock();
    stdout.flush().unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use llm::Model;
    use std::sync::Arc;
    env_logger::init();

    let llama = load_language_model();
    println!("model loaded");

    // use the model to generate text from a prompt
    let mut session = Model::start_session(&llama, Default::default());

    println!("Started session");

    static CHARACTER_NAME: &str = "### Assistant";
    static USER_NAME: &str = "### Human";

    let persona = "A chat between a human and an assistant.";
    let history = format!(
        "{CHARACTER_NAME}:Hello - How may I help you today?\n\
                {USER_NAME}:What is the capital of France?\n\
                {CHARACTER_NAME}:Paris is the capital of France.\n"
    );

    println!("Set up initial prompt");
    let llama = Arc::new(llama);
    // let session = session_setup(llama.clone());
    let model = web::Data::new(llama);

    println!("Host websocket api!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(model.clone())
            .service(chat_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
