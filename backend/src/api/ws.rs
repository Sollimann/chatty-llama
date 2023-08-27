use crate::handler::model::{run_inference, session_setup};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use llm::{models::Llama, InferenceSession};
use std::sync::Arc;
use tokio_stream::StreamExt;

pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    model: web::Data<Arc<Llama>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let model = model.as_ref().clone();
    let model_clone = model.clone();

    let inference_session: InferenceSession =
        web::block(move || session_setup(model)).await.unwrap();

    let inference_session = Arc::new(std::sync::Mutex::new(inference_session));

    println!("Initialized inference session");
    actix_rt::spawn(async move {
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

                    println!("Running inference...");

                    let inference_result = web::block(move || {
                        let mut locked_session = session_clone.lock().unwrap();
                        run_inference(&model_for_this_iteration, &mut *locked_session, &text)
                    })
                    .await
                    .unwrap();

                    println!("Finished inference!");

                    let bot_response = format!("Bot: {:?}", inference_result);
                    if session.text(bot_response).await.is_err() {
                        return;
                    }
                }
                _ => break,
            }
        }

        let _ = session.close(None).await;
        println!("Disconnecting websocket!");
    });

    Ok(response)
}
