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
    tokio_tungstenite::{
        accept_async,
        tungstenite::{self, Error},
    },
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
    info!("Opened connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        match ws_receiver.next().await {
            Some(msg) => {
                let msg = msg?;

                if msg.is_text() {
                    debug!("Received text message: {:?}", msg);
                    let response = match serde_json::from_str::<ClientMessage>(&msg.into_text()?) {
                        Ok(ClientMessage::Create { username }) => {
                            tungstenite::Message::text("test")
                        }
                        Ok(ClientMessage::Join { username, game_id }) => {
                            tungstenite::Message::text("test")
                        }
                        Ok(ClientMessage::Move(m)) => tungstenite::Message::text("test"),
                        Err(e) => {
                            error!("Error occurred while parsing: {:?}", e);
                            break;
                        }
                    };
                    ws_sender.send(response).await?;
                } else {
                    error!("Received invalid data: {:?}", msg);
                    break;
                }
            }
            None => break, // WebSocket stream terminated.
        };
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

    let addr = "0.0.0.0:22220";
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

/*
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
*/
