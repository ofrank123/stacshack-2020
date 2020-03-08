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
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            resources: 0,
            color: thread_rng().gen_range(0x00000000, 0x00ffffff) + 0xff000000,
            current: false,
        }
    }
}
