use {
    crate::{board::Board, player::Player},
    chrono::prelude::*,
    serde::{Deserialize, Serialize},
    tungstenite::Message,
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { game_id: u16, username: String },
    Create { username: String },
    Action(Option<Action>),
}

impl From<ClientMessage> for Message {
    fn from(msg: ClientMessage) -> Self {
        Self::binary(
            serde_json::to_string(&msg)
                .expect("Failed to serialize ServerMessage")
                .into_bytes(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub user_id: Uuid,
    pub kind: ActionKind,
    pub coordinate: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<ServerMessage> for Message {
    fn from(msg: ServerMessage) -> Self {
        Self::binary(
            serde_json::to_string(&msg)
                .expect("Failed to serialize ServerMessage")
                .into_bytes(),
        )
    }
}
