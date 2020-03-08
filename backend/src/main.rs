#[macro_use]
extern crate log;

mod board;
mod game;
mod message;
mod player;

use {
    board::TileKind,
    chrono::prelude::*,
    game::Game,
    message::{ClientMessage, ServerMessage},
    rand::{thread_rng, Rng},
    std::{
        collections::HashMap,
        env,
        net::{TcpListener, TcpStream},
        sync::{
            mpsc::{channel, Receiver, Sender},
            Arc, Mutex,
        },
        thread::{sleep, spawn},
        time::Duration,
    },
    tungstenite::{
        accept,
        handshake::server::{Request, Response},
        Message, Result, WebSocket,
    },
    uuid::Uuid,
};

const TIMEOUT_MS: u64 = 15_000;
const RESOURCE_CHANCE: f64 = 0.10;
const GAME_SIZE: usize = 2;
const BOARD_SIZE: usize = 16;
const COLORS: [u32; 4] = [0x32a852ff, 0x9e32a8ff, 0xff0000ff, 0xfbff00ff];

struct State {
    games: HashMap<u16, Game>,
    connections: HashMap<u16, Vec<WebSocket<TcpStream>>>,
}

fn game_handler(game_id: u16, state: Arc<Mutex<State>>) -> Result<()> {
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
        sleep(Duration::from_millis(10));
    }

    info!("All players connected, starting game!");

    let mut game = state.lock().unwrap().games.remove(&game_id).unwrap();
    let mut connections = state.lock().unwrap().connections.remove(&game_id).unwrap();

    let mut current_player = 0;
    game.players[current_player].current = true;

    let mut rng = thread_rng();
    for (i, player) in game.players.iter_mut().enumerate() {
        let x = rng.gen_range(0, BOARD_SIZE);
        let y = rng.gen_range(0, BOARD_SIZE);

        let tile = game.board.get_mut_tile(x, y);
        tile.kind = if rng.gen::<f64>() < RESOURCE_CHANCE {
            TileKind::Resource
        } else {
            TileKind::Normal
        };
        tile.owner = Some(player.id);

        player.color = COLORS[i];
    }

    debug!("set current player");

    for ws in &mut connections {
        ws.write_message(
            ServerMessage::Action {
                last_action: None,
                board: game.board.clone(),
                players: game.players.clone(),
                expiry: Utc::now() + chrono::Duration::seconds(15),
            }
            .into(),
        )?;
    }

    debug!("sent initial state");

    //TODO timout

    loop {
        let msg = connections[current_player].read_message()?;
        if msg.is_binary() {
            match serde_json::from_str::<ClientMessage>(
                &msg.into_text().expect("Unable to turn message into text"),
            ) {
                Ok(ClientMessage::Action(action)) => {
                    debug!("received action: {:?}", action);
                    if action.is_some() {
                        game.action(&action.clone().unwrap());
                    }

                    game.players[current_player].current = false;
                    current_player = (current_player + 1) % GAME_SIZE;
                    game.players[current_player].current = true;

                    for ws in &mut connections {
                        ws.write_message(
                            ServerMessage::Action {
                                last_action: action.clone(),
                                board: game.board.clone(),
                                players: game.players.clone(),
                                expiry: Utc::now() + chrono::Duration::seconds(15),
                            }
                            .into(),
                        )?;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

fn connection_handler(mut ws: WebSocket<TcpStream>, state: Arc<Mutex<State>>) -> Result<()> {
    debug!("Handling new connection: {:?}", ws);

    let msg = ws.read_message()?;
    if msg.is_binary() {
        match serde_json::from_str::<ClientMessage>(
            &msg.into_text().expect("Unable to turn message into text"),
        ) {
            Ok(ClientMessage::Create { username }) => {
                let mut game = Game::new(BOARD_SIZE);
                let game_id = thread_rng().gen();
                let user_id = game.add_player(&username);

                state.lock().unwrap().games.insert(game_id, game);

                ws.write_message(ServerMessage::Create { game_id, user_id }.into())
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

                ws.write_message(ServerMessage::Join { user_id }.into())
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

    Ok(())
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
