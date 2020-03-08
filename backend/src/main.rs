#[macro_use]
extern crate log;

mod board;
mod game;
mod message;
mod player;

use {
    chrono::prelude::*,
    game::Game,
    message::{ClientMessage, ServerMessage},
    rand::{thread_rng, Rng},
    std::{
        collections::HashMap,
        env,
        net::{TcpListener, TcpStream},
        sync::{Arc, Mutex},
        thread::{sleep, spawn},
        time::Duration,
    },
    tungstenite::{
        accept,
        handshake::server::{Request, Response},
        Message, WebSocket,
    },
    uuid::Uuid,
};

const TIMEOUT_MS: u64 = 15_000;
const RESOURCE_CHANCE: f64 = 0.10;
const GAME_SIZE: usize = 2;

struct State {
    games: HashMap<u16, Game>,
    connections: HashMap<u16, Vec<WebSocket<TcpStream>>>,
}

fn game_handler(game_id: u16, state: Arc<Mutex<State>>) {
    // wait until 4 connections are in state then continue
    while state
        .lock()
        .unwrap()
        .connections
        .get(&game_id)
        .unwrap()
        .len()
        != GAME_SIZE
    {
        sleep(Duration::from_millis(1000));
    }

    info!("All players connected, starting game!");

    let mut game = state.lock().unwrap().games.remove(&game_id).unwrap();
    let mut connections = state.lock().unwrap().connections.remove(&game_id).unwrap();

    // Send initial state to all players
    let mut current_player = 0;
    game.players[current_player].current = true;

    debug!("set current player");

    for mut ws in connections {
        ws.write_message(Message::text(
            &serde_json::to_string(&ServerMessage::Action {
                last_action: None,
                board: game.board.clone(),
                players: game.players.clone(),
                expiry: Utc::now() + chrono::Duration::seconds(15),
            })
            .expect("Failed to serialize ServerMessage"),
        ))
        .expect("Failed to write message");
    }

    debug!("sent all initial messages");
}

fn connection_handler(mut ws: WebSocket<TcpStream>, state: Arc<Mutex<State>>) {
    debug!("Handling new connection: {:?}", ws);
    // create a new game and put connection in global state
    // or
    // place connection in existing state

    let msg = ws.read_message().unwrap();
    if msg.is_binary() {
        match serde_json::from_str::<ClientMessage>(
            &msg.into_text().expect("Unable to turn message into text"),
        ) {
            Ok(ClientMessage::Create { username }) => {
                let mut game = Game::new(16);
                let game_id = thread_rng().gen();
                let user_id = game.add_player(&username);

                state.lock().unwrap().games.insert(game_id, game);

                ws.write_message(Message::text(
                    &serde_json::to_string(&ServerMessage::Create { game_id, user_id })
                        .expect("Failed to serialize ServerMessage"),
                ))
                .expect("Failed to write message");

                state.lock().unwrap().connections.insert(game_id, vec![ws]);

                spawn(move || game_handler(game_id, state.clone()));
            }
            Ok(ClientMessage::Join { game_id, username }) => {
                let user_id = state
                    .lock()
                    .unwrap()
                    .games
                    .get_mut(&game_id)
                    .unwrap()
                    .add_player(&username);

                ws.write_message(Message::text(
                    &serde_json::to_string(&ServerMessage::Join { user_id })
                        .expect("Failed to serialize ServerMessage"),
                ))
                .expect("Failed to write message");

                // add socket to game connections
                state
                    .lock()
                    .unwrap()
                    .connections
                    .get_mut(&game_id)
                    .unwrap()
                    .push(ws);
            }
            _ => unimplemented!(),
        }
    }
}

fn main() {
    // Logging
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Global State
    let state = Arc::new(Mutex::new(State {
        games: HashMap::new(),
        connections: HashMap::new(),
    }));

    // Server
    let server = TcpListener::bind("0.0.0.0:22220").unwrap();
    info!("Started server at ws://0.0.0.0:22220");
    for stream in server.incoming() {
        let state = state.clone();
        spawn(move || connection_handler(accept(stream.unwrap()).unwrap(), state));
    }
}

/*
let client_msg1 = ClientMessage::Create {
        username: "Hunter2".to_string(),
    };

    let client_msg2 = ClientMessage::Join {
        username: "Hunter2".to_string(),
        game_id: thread_rng().gen(),
    };

    let client_msg3 = ClientMessage::Action(message::Action {
        user_id: Uuid::new_v4(),
        kind: message::ActionKind::Attack,
        coordinate: (12, 41),
    });

    println!("{}", serde_json::to_string_pretty(&client_msg1).unwrap());
    println!("{}", serde_json::to_string_pretty(&client_msg2).unwrap());
    println!("{}", serde_json::to_string_pretty(&client_msg3).unwrap());

    let server_msg1 = ServerMessage::Create {
        game_id: thread_rng().gen(),
        user_id: Uuid::new_v4(),
    };

    let server_msg2 = ServerMessage::Join {
        user_id: Uuid::new_v4(),
    };

    let server_msg3 = ServerMessage::Action {
        last_action: Some(message::Action {
            user_id: Uuid::new_v4(),
            kind: message::ActionKind::Attack,
            coordinate: (12, 41),
        }),
        board: board::Board::new(16),
        players: vec![
            player::Player::new("hunter2"),
            player::Player::new("player2"),
            player::Player::new("test"),
            player::Player::new("john"),
        ],
        expiry: Utc::now() + chrono::Duration::seconds(15),
    };

    println!("{}", serde_json::to_string_pretty(&server_msg1).unwrap());
    println!("{}", serde_json::to_string_pretty(&server_msg2).unwrap());
    println!("{}", serde_json::to_string_pretty(&server_msg3).unwrap());

    std::process::exit(0);
    */
