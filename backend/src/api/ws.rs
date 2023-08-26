use crate::handler::model::run_inference;
use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use llm::models::Llama;
use std::sync::Arc;

/// Define our WebSocket actor
pub struct Conversation {
    pub messages: Vec<Message>,
    llama: Arc<Llama>,
}

pub struct Message {
    pub user: bool,
    pub text: String,
}

impl Actor for Conversation {
    type Context = ws::WebsocketContext<Self>;
}

/// Handle incoming messages for WebSocket actor
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Conversation {
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
        Conversation {
            llama: model.as_ref().clone(),
            messages: vec![],
        },
        &req,
        stream,
    )
}
