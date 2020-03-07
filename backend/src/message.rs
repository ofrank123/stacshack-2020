use {
    crate::{board::Board, player::Player},
    chrono::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { username: String, game_id: Uuid },
    Create { username: String },
    Move(Move),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub action: Action,
    pub coordinate: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Explore,
    Improve,
    Attack,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Join {
        user_id: Uuid,
    },
    Create {
        user_id: Uuid,
        game_id: Uuid,
    },
    Move {
        last_move: Move,
        board: Board,
        players: Vec<Player>,
        expiry: DateTime<Utc>,
    },
}
