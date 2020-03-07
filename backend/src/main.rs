#[macro_use]
extern crate log;

mod board;
mod game;
mod message;
mod player;

use {
    board::Board,
    chrono::prelude::*,
    futures_util::future::{select, Either},
    futures_util::{SinkExt, StreamExt},
    game::Game,
    message::{ClientMessage, ServerMessage},
    player::Player,
    slog::{o, Fuse},
    std::{
        collections::HashMap,
        io,
        net::SocketAddr,
        sync::{Arc, Mutex},
        time::Duration,
    },
    tokio::net::{TcpListener, TcpStream},
    tokio_tungstenite::{accept_async, tungstenite::Error},
    tungstenite::{Message, Result},
    uuid::Uuid,
};

type State = Arc<Mutex<HashMap<Uuid, Game>>>;

const TIMEOUT_MS: usize = 15_000;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, state: State) {
    if let Err(e) = handle_connection(peer, stream, state).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, state: State) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.

    let mut msg_fut = ws_receiver.next();
    let mut tick_fut = interval.next();
    loop {
        match select(msg_fut, tick_fut).await {
            Either::Left((msg, tick_fut_continue)) => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                        tick_fut = tick_fut_continue; // Continue waiting for tick.
                        msg_fut = ws_receiver.next(); // Receive next WebSocket message.
                    }
                    None => break, // WebSocket stream terminated.
                };
            }
            Either::Right((_, msg_fut_continue)) => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
                msg_fut = msg_fut_continue; // Continue receiving the WebSocket message.
                tick_fut = interval.next(); // Wait for next tick.
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let client_msg1 = ClientMessage::Create {
        username: "Hunter2".to_string(),
    };

    let client_msg2 = ClientMessage::Join {
        username: "Hunter2".to_string(),
        game_id: Uuid::new_v4(),
    };

    let client_msg3 = ClientMessage::Move(message::Move {
        action: message::Action::Attack,
        coordinate: (12, 41),
    });

    println!("{}", serde_json::to_string_pretty(&client_msg1).unwrap());
    println!("{}", serde_json::to_string_pretty(&client_msg2).unwrap());
    println!("{}", serde_json::to_string_pretty(&client_msg3).unwrap());

    let server_msg1 = ServerMessage::Create {
        game_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
    };

    let server_msg2 = ServerMessage::Join {
        user_id: Uuid::new_v4(),
    };

    let server_msg3 = ServerMessage::Move {
        last_move: message::Move {
            action: message::Action::Attack,
            coordinate: (12, 41),
        },
        board: Board::new(16),
        players: vec![Player::new(Uuid::new_v4())],
        expiry: Utc::now() + chrono::Duration::seconds(15),
    };

    println!("{}", serde_json::to_string_pretty(&server_msg1).unwrap());
    println!("{}", serde_json::to_string_pretty(&server_msg2).unwrap());
    println!("{}", serde_json::to_string_pretty(&server_msg3).unwrap());

    std::process::exit(0);

    // Logging
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog_async::Async::new(Fuse(drain)).build();
    let logger = slog::Logger::root(Fuse(drain), o!("version" => env!("CARGO_PKG_VERSION")));
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init_with_level(log::Level::Debug).unwrap();

    // Game State
    let state = Arc::new(Mutex::new(HashMap::<Uuid, Game>::new()));

    let addr = "127.0.0.1:8088";
    let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, state.clone()));
    }
}
