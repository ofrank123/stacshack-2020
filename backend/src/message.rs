use {
    crate::{board::Board, player::Player},
    chrono::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { game_id: u16, username: String },
    Create { username: String },
    Action(Action),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub user_id: Uuid,
    pub kind: ActionKind,
    pub coordinate: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionKind {
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
        game_id: u16,
    },
    Action {
        last_action: Option<Action>,
        board: Board,
        players: Vec<Player>,
        expiry: DateTime<Utc>,
    },
}
