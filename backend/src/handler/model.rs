use dotenv::dotenv;
use llm::models::Llama;
use std::convert::Infallible;
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

pub fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    out_str: &'a mut String,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Continue;
    use llm::InferenceFeedback::Halt;

    move |resp| match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
            reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok::<llm::InferenceFeedback, Infallible>(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }

            if buf.is_empty() {
                out_str.push_str(&t);
            } else {
                out_str.push_str(&reverse_buf);
            }

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
    }
}
