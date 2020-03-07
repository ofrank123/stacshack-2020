use {
    rand::{thread_rng, Rng},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub resources: u32,
    pub color: u32,
    pub current: bool,
}

impl Player {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            resources: 0,
            color: thread_rng().gen_range(0x00000000, 0x00ffffff) + 0xff000000,
            current: false,
        }
    }
}
