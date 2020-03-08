use {
    crate::{
        board::{Board, Defence, TileKind},
        message::{Action, ActionKind},
        player::Player,
    },
    rand::{thread_rng, Rng},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            board: Board::new(size),
            players: vec![],
        }
    }

    pub fn add_player(&mut self, username: String) -> Uuid {
        let player = Player::new(username);
        let id = player.id;

        self.players.push(player);

        id
    }

    pub fn action(&mut self, action: &Action) {
        let tile = self
            .board
            .get_mut_tile(action.coordinate.0, action.coordinate.1);

        match action.kind {
            ActionKind::Explore => {
                if tile.kind != TileKind::Hidden {
                    error!("Trying to explore an already explored tile");
                    return;
                }

                tile.kind = if thread_rng().gen::<f64>() < crate::RESOURCE_CHANCE {
                    TileKind::Resource
                } else {
                    TileKind::Normal
                };
            }
            ActionKind::Improve => {
                if tile.owner != Some(action.user_id) {
                    error!("Trying to improve a tile owned by another player");
                    return;
                }

                if tile.defence == Defence::High {
                    error!("Trying to improve a tile already at maximum improvement");
                    return;
                }

                tile.increase_defence();
                //TODO remove points from player
            }
            ActionKind::Attack => {
                //TODO properly implement attack

                if tile.owner == Some(action.user_id) {
                    error!("Cannot attack own tile");
                    return;
                }

                tile.owner = Some(action.user_id);
            }
        }
    }
}
