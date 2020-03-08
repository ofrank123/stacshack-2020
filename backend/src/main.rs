#[macro_use]
extern crate log;

mod board;
mod game;
mod message;
mod player;

use {
    chrono::prelude::*,
    futures_util::{
        future::{select, select_all, Either},
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    },
    game::Game,
    message::{ClientMessage, ServerMessage},
    rand::{thread_rng, Rng},
    slog::{o, Fuse},
    std::{
        collections::HashMap,
        net::SocketAddr,
        sync::{Arc, Mutex},
    },
    tokio::net::{TcpListener, TcpStream},
    tokio_tungstenite::{
        accept_async,
        tungstenite::{self, Error, Message, Result},
        WebSocketStream,
    },
    uuid::Uuid,
};

const TIMEOUT_MS: u64 = 15_000;
const RESOURCE_CHANCE: f64 = 0.10;
const GAME_SIZE: usize = 4;

struct State {
    games: HashMap<u16, Game>,
    connections: HashMap<u16, Vec<WebSocketStream<TcpStream>>>,
}

async fn game_handler(
    mut game: Game,
    mut connections: Vec<WebSocketStream<TcpStream>>,
) -> Result<()> {
    let mut current_player = 0;
    game.players[current_player].current = true;

    // Send initial state to all clients
    for e in connections.iter_mut() {
        e.send(Message::text(
            &serde_json::to_string(&ServerMessage::Action {
                last_action: None,
                board: game.board.clone(),
                players: game.players.clone(),
                expiry: Utc::now() + chrono::Duration::seconds(15),
            })
            .expect("Failed to serialize ServerMessage"),
        ))
        .await?
    }

    let mut timeout = tokio::time::interval(std::time::Duration::from_millis(TIMEOUT_MS));

    let mut timeout_fut = timeout.next();

    loop {
        match select(
            select_all(connections.iter_mut().map(|s| s.next())),
            timeout_fut,
        )
        .await
        {
            Either::Left(((msg, n, conn), timeout_fut_continue)) => {
                let msg = msg.unwrap().unwrap();
                if msg.is_binary() {
                    debug!("Received binary message: {:?}", msg);
                    match serde_json::from_str::<ClientMessage>(&msg.into_text()?) {
                        Ok(ClientMessage::Action(action)) => {
                            info!("Received Action: {:?}", action);
                            game.action(&action);

                            game.players[current_player].current = false;
                            current_player = (current_player + 1) % GAME_SIZE;
                            game.players[current_player].current = true;

                            timeout_fut = timeout_fut_continue;
                        }
                        Err(e) => {
                            error!("Error occurred while parsing: {:?}", e);
                            break;
                        }
                        msg => {
                            error!("Invalid ClientMessage received: {:?}", msg);
                            break;
                        }
                    }
                } else if msg.is_close() {
                    info!("Closing: {:?}", msg);
                    break;
                } else {
                    error!("Received invalid data: {:?}", msg);
                    break;
                }
            }
            Either::Right((_, timeout_fut_continue)) => {
                info!("Timout!");
                timeout_fut = timeout.next();
            }
        }

        for e in connections.iter_mut() {
            e.send(Message::text(
                &serde_json::to_string(&ServerMessage::Action {
                    last_action: None,
                    board: game.board.clone(),
                    players: game.players.clone(),
                    expiry: Utc::now() + chrono::Duration::seconds(15),
                })
                .expect("Failed to serialize ServerMessage"),
            ))
            .await?
        }
    }

    Ok(())
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, state: Arc<Mutex<State>>) {
    if let Err(e) = handle_connection(peer, stream, state).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    state: Arc<Mutex<State>>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("Opened connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    match ws_receiver.next().await {
        Some(msg) => {
            let msg = msg?;

            if msg.is_binary() {
                debug!("Received binary message: {:?}", msg);
                match serde_json::from_str::<ClientMessage>(&msg.into_text()?) {
                    Ok(ClientMessage::Create { username }) => {
                        // Create a new Game and add an initial player
                        let mut game = Game::new(16);
                        let game_id = thread_rng().gen();
                        let user_id = game.add_player(username);

                        state.lock().unwrap().games.insert(game_id, game);

                        ws_sender
                            .send(Message::text(
                                &serde_json::to_string(&ServerMessage::Create { game_id, user_id })
                                    .expect("Failed to serialize ServerMessage"),
                            ))
                            .await?;

                        state.lock().unwrap().connections.insert(
                            game_id,
                            vec![ws_receiver
                                .reunite(ws_sender)
                                .expect("Unable to reunite split socket")],
                        );
                    }
                    Ok(ClientMessage::Join { game_id, username }) => {
                        // Add a new player to the existing Game
                        let user_id = state
                            .lock()
                            .unwrap()
                            .games
                            .get_mut(&game_id)
                            .unwrap()
                            .add_player(username);

                        //TODO NEED TO TEST HERE WHEN 4TH PLAYER JOINS

                        ws_sender
                            .send(Message::text(
                                &serde_json::to_string(&ServerMessage::Join { user_id })
                                    .expect("Failed to serialize ServerMessage"),
                            ))
                            .await?;

                        {
                            let mut state = state.lock().unwrap();
                            state
                                .connections
                                .get_mut(&game_id)
                                .expect("Joining non-existant game")
                                .push(
                                    ws_receiver
                                        .reunite(ws_sender)
                                        .expect("Unable to reunite split socket"),
                                );

                            if state.connections.get_mut(&game_id).unwrap().len() == GAME_SIZE {
                                let connections = state.connections.remove(&game_id).unwrap();
                                let game = state.games.remove(&game_id).unwrap();
                                tokio::spawn(game_handler(game, connections));
                            }
                        }
                    }
                    Ok(ClientMessage::Action(action)) => {
                        error!(
                            "Action message recieved but not allowed outside of game, action: {:?}",
                            action
                        );
                    }
                    Err(e) => {
                        error!("Error occurred while parsing: {:?}", e);
                    }
                };
            } else if msg.is_close() {
                info!("Closing: {:?}", msg);
            } else {
                error!("Received invalid data: {:?}", msg);
            }
        }
        None => (), // WebSocket stream terminated
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    // Logging
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog_async::Async::new(Fuse(drain)).build();
    let logger = slog::Logger::root(Fuse(drain), o!("version" => env!("CARGO_PKG_VERSION")));
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init_with_level(log::Level::Debug).unwrap();

    // Game State
    let state = Arc::new(Mutex::new(State {
        games: HashMap::new(),
        connections: HashMap::new(),
    }));

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
fn main() {
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
}
*/
