use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use futures::StreamExt;
use tokio::sync::mpsc;
use twitter_stream::{Token, TwitterStreamBuilder};
use std::env;
use dotenv::dotenv;

async fn websocket_route(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(WebSocketActor::new(), &req, stream)
}

struct WebSocketActor {
    sender: Option<mpsc::UnboundedSender<String>>,
}

impl WebSocketActor {
    fn new() -> Self {
        WebSocketActor { sender: None }
    }
}

impl actix::Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        dotenv().ok();

        let consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY must be set");
        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET must be set");
        let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN must be set");
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET must be set");

        let (sender, receiver) = mpsc::unbounded_channel();
        self.sender = Some(sender);

        let stream = TwitterStreamBuilder::new()
            .token(Token::new(&consumer_key, &consumer_secret, &access_token, &access_token_secret))
            .track("rust")
            .listen()
            .await
            .unwrap();

        ctx.spawn(async move {
            let mut stream = stream.boxed();
            while let Some(Ok(json)) = stream.next().await {
                if let Some(sender) = self.sender.as_ref() {
                    sender.send(json.to_string()).unwrap();
                }
            }
        });

        let sender = self.sender.clone().unwrap();
        ctx.spawn(receiver.for_each(move |msg| {
            ctx.text(msg);
            actix::fut::ready(())
        }));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ws/").route(web::get().to(websocket_route)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../frontend/index.html"))
}