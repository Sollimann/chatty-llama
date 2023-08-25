use dotenv::dotenv;
use llm::models::Llama;
use std::env;

pub fn load_language_model() -> Llama {
    use std::path::PathBuf;
    dotenv().ok();
    let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");
    let model_parameters = llm::ModelParameters::default();

    llm::load::<Llama>(
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        model_parameters,
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model from {model_path:?}: {err}"))
}
