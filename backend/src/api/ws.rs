use crate::handler::model::{run_inference, session_setup};
use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use llm::{models::Llama, InferenceSession};
use std::sync::Arc;

/// Define our WebSocket actor
pub struct Conversation {
    pub messages: Vec<Message>,
    llama: Arc<Llama>,
    session: InferenceSession,
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
            println!("Running inference...");
            let inference_result = run_inference(&self.llama, &mut self.session, &text);
            println!("Inference complete!");

            // Send the inference result back to the client
            let response = format!("Bot: {}", inference_result);
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
    println!("ws endpoint!");
    let model = model.as_ref().clone();
    println!("init session");
    let session = session_setup(model.clone());

    ws::start(
        Conversation {
            llama: model,
            messages: vec![],
            session,
        },
        &req,
        stream,
    )
}
