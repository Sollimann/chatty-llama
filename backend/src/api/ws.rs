use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use llm::models::Llama;
use std::sync::Arc;

/// Define our WebSocket actor
struct MyWs {
    llama: Arc<Llama>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

// A mock function to represent inference (replace with actual implementation)
fn run_inference(model: &Llama, text: &str) -> String {
    // Replace this with the actual model inference code
    format!("Bot: Inference for {}: {}", text, "some result from model")
}

/// Handle incoming messages for WebSocket actor
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            // Use the model for inference
            let inference_result = run_inference(&self.llama, &text);
            println!("inference res: {inference_result}");

            // Send the inference result back to the client
            let response = format!("Bot: {}", text);
            ctx.text(response);
        }
    }
}

#[get("/ws/")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    model: web::Data<Arc<Llama>>,
) -> Result<HttpResponse, Error> {
    // let m = model.as_ref();
    println!("ws endpoint!");
    ws::start(
        MyWs {
            llama: model.as_ref().clone(),
        },
        &req,
        stream,
    )
}
