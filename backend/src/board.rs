use {
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    size: usize,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut tiles = Vec::with_capacity(size * size);
        for _ in 0..(size * size) {
            tiles.push(Tile::default());
        }
        Self { size, tiles }
    }

    pub fn get_mut_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[x * self.size + y]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub owner: Option<Uuid>,
    pub defence: Defence,
}

impl Tile {
    pub fn increase_defence(&mut self) {
        self.defence = match self.defence {
            Defence::None => Defence::Low,
            Defence::Low => Defence::High,
            Defence::High => {
                return;
            }
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: TileKind::Hidden,
            owner: None,
            defence: Defence::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TileKind {
    Hidden,
    Normal,
    Resource,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Defence {
    None,
    Low,
    High,
}
