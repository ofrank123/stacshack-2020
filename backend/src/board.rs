use {
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x * self.size + y]
    }

    pub fn get_mut_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[x * self.size + y]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub kind: Kind,
    pub owner: Option<Uuid>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: Kind::Empty,
            owner: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    Empty,
    Resource,
}
