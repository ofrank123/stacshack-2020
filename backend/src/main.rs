#[macro_use]
extern crate log;

mod board;
mod game;
mod message;
mod player;

use {
    actix::{Actor, StreamHandler},
    actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer},
    actix_web_actors::ws::{self, CloseCode, CloseReason},
    board::Board,
    game::Game,
    player::Player,
    serde_json,
    slog::{o, Fuse},
    std::{
        collections::HashMap,
        io,
        sync::{Arc, Mutex},
    },
    uuid::Uuid,
};

struct WebsocketActor {
    game_state: Arc<Mutex<HashMap<Uuid, Game>>>,
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                info!(
                    "Received text data: {:?}, state: {:?}",
                    text, self.game_state
                );

                ctx.text("Response!")
            }
            _ => {
                error!("Received invalid WebSocket data, closing");
                ctx.close(Some(CloseReason {
                    code: CloseCode::Unsupported,
                    description: None,
                }));
            }
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Mutex<HashMap<Uuid, Game>>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebsocketActor {
            game_state: data.into_inner(),
        },
        &req,
        stream,
    )
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog_async::Async::new(Fuse(drain)).build();
    let logger = slog::Logger::root(Fuse(drain), o!("version" => env!("CARGO_PKG_VERSION")));
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init_with_level(log::Level::Debug).unwrap();

    // Game State
    let game_state = web::Data::new(Mutex::new(HashMap::<Uuid, Game>::new()));

    // Server
    HttpServer::new(move || {
        App::new()
            .app_data(game_state.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
