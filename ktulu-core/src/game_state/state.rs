use game_state::{GameStage, Player};
use std::collections::HashMap;
use PlayerId;

#[derive(Clone, Copy, Debug)]
pub enum GameEntity {
    Manitou,
    Player(PlayerId),
}

#[derive(Clone, Debug)]
pub struct GameState {
    us: GameEntity,
    players: HashMap<PlayerId, Player>,
    stage: GameStage,
}

impl GameState {
    pub fn new_manitou() -> Self {
        Self {
            us: GameEntity::Manitou,
            players: HashMap::new(),
            stage: GameStage::new(),
        }
    }

    pub fn new_player(id: PlayerId) -> Self {
        Self {
            us: GameEntity::Player(id),
            players: HashMap::new(),
            stage: GameStage::new(),
        }
    }
}
