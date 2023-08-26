pub mod api;
pub mod handler;

use std::io::Write;

use actix_web::{App, HttpServer};
use api::ws::chat_route;
use handler::model::{inference_callback, load_language_model};

fn print_token(t: String) {
    print!("{t}");
    let mut stdout = std::io::stdout().lock();
    stdout.flush().unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use llm::Model;

    let llama = load_language_model();
    println!("model loaded");

    // use the model to generate text from a prompt
    let mut session = Model::start_session(&llama, Default::default());

    println!("Started session");
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    static CHARACTER_NAME: &str = "### Assistant";
    static USER_NAME: &str = "### Human";

    let persona = "A chat between a human and an assistant.";
    let history = format!(
        "{CHARACTER_NAME}:Hello - How may I help you today?\n\
                {USER_NAME}:What is the capital of France?\n\
                {CHARACTER_NAME}:Paris is the capital of France.\n"
    );

    println!("Set up initial prompt");
    session
        .feed_prompt(
            &llama,
            format!("{persona}\n{history}").as_str(),
            &mut Default::default(),
            llm::feed_prompt_callback(|resp| match resp {
                llm::InferenceResponse::PromptToken(t)
                | llm::InferenceResponse::InferredToken(t) => {
                    print_token(t);

                    Ok::<llm::InferenceFeedback, std::convert::Infallible>(
                        llm::InferenceFeedback::Continue,
                    )
                }
                _ => Ok(llm::InferenceFeedback::Continue),
            }),
        )
        .expect("Failed to ingest initial prompt.");

    // session
    //     .infer(
    //         &llama,
    //         &mut rng,
    //         &llm::InferenceRequest {
    //             prompt: format!("{persona}\n{history}\n{CHARACTER_NAME}:")
    //                 .as_str()
    //                 .into(),
    //             parameters: &llm::InferenceParameters::default(),
    //             play_back_previous_tokens: false,
    //             maximum_token_count: None,
    //         },
    //         &mut Default::default(),
    //         inference_callback(String::from(USER_NAME), &mut buf, &mut res),
    //     )
    //     .unwrap_or_else(|e| panic!("{e}"));

    // println!("Res: {res}");

    HttpServer::new(move || App::new().service(chat_route))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
