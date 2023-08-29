use crate::handler::model::{run_inference, session_setup};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use llm::{models::Llama, InferenceSession};
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tokio_stream::StreamExt;

pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    model: web::Data<Arc<Llama>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let model = model.as_ref().clone();
    let model_clone = model.clone();

    println!("Started websocket connection...");
    actix_rt::spawn(async move {
        println!("Preparing inference model...");
        let inference_session: InferenceSession =
            web::block(move || session_setup(model)).await.unwrap();

        let inference_session = Arc::new(std::sync::Mutex::new(inference_session));
        println!("Initialized inference session.");

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => {
                    let model_for_this_iteration = model_clone.clone();
                    let session_clone = inference_session.clone();

                    log::info!("Running inference...");
                    let inference_result = timeout(
                        Duration::from_secs(40),
                        web::block(move || {
                            let mut locked_session = session_clone.lock().unwrap();
                            run_inference(&model_for_this_iteration, &mut *locked_session, &text)
                        }),
                    )
                    .await;

                    match inference_result {
                        Ok(Ok(result)) => {
                            log::info!("Finished inference!");
                            let bot_response = format!("{result:?}");
                            let bot_response =
                                bot_response.trim_start_matches('"').trim_end_matches('"');
                            if session.text(bot_response).await.is_err() {
                                return;
                            }
                        }
                        Ok(Err(e)) => {
                            log::error!("Error during inference: {:?}", e);
                            // Handle this error according to your needs
                            return;
                        }
                        Err(_) => {
                            log::error!("Request timed out after 40 seconds.");
                            // Handle timeout error, for example by sending a timeout message to the client
                            if session
                                .text("Request timed out after 40 seconds... Try again!")
                                .await
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    log::info!("Client requested close. Cleaning up.");
                    // Do cleanup if needed
                    break;
                }
                _ => break,
            }
        }

        let _ = session.close(None).await;
        println!("Disconnecting websocket!");
    });

    println!("Started websocket connection...");
    Ok(response)
}
