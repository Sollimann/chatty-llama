use dotenv::dotenv;
use llm::models::Llama;
use llm::Model;
use std::convert::Infallible;
use std::env;
use std::sync::Arc;

static CHARACTER_NAME: &str = "### Assistant";
static USER_NAME: &str = "### Human";

// Load local model from path
pub fn load_language_model() -> Llama {
    use std::path::PathBuf;
    dotenv().ok();

    let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");

    let model_parameters = llm::ModelParameters {
        prefer_mmap: true,
        context_size: 2048,
        lora_adapters: None,
        use_gpu: true,
        gpu_layers: None,
        rope_overrides: None,
        n_gqa: None,
    };

    llm::load::<Llama>(
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        model_parameters,
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model from {model_path:?}: {err}"))
}

// Create a session based of a local modal
pub(crate) fn session_setup(model: Arc<Llama>) -> llm::InferenceSession {
    let persona = "A chat between a human and an assistant.";
    let history = format!(
        "{CHARACTER_NAME}:Hello - How may I help you today?\n\
                {USER_NAME}:What is the capital of France?\n\
                {CHARACTER_NAME}:Paris is the capital of France.\n"
    );

    let mut session = model.start_session(Default::default());
    session
        .feed_prompt(
            model.as_ref(),
            format!("{persona}\n{history}").as_str(),
            &mut Default::default(),
            llm::feed_prompt_callback(|_| {
                Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
            }),
        )
        .expect("Failed to ingest initial prompt.");

    session
}

// Logic to decide how long the model should do inference
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

// Run actual model inference
pub(crate) fn run_inference(
    model: &Llama,
    inference_session: &mut llm::InferenceSession,
    user_message: &str,
) -> String {
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    inference_session
        .infer(
            model,
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{USER_NAME}: {user_message}\n{CHARACTER_NAME}:")
                    .as_str()
                    .into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(USER_NAME), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    res
}
