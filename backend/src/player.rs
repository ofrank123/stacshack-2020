use {
    rand::{thread_rng, Rng},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub resources: u32,
    pub color: u32,
    pub current: bool,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            resources: 0,
            color: 0,
            current: false,
        }
    }
}
