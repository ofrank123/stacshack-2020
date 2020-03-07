use {
    crate::game::Game,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub game: Uuid,
    pub user: Uuid,
    pub kind: Kind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    Join,
    Fetch,
    Action {
        kind: Action,
        coordinate: (usize, usize),
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Explore,
    Improve,
    Attack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub Game);
