use {
    crate::{board::Board, player::Player},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
}

impl Game {}
