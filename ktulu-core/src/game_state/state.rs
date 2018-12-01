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

    pub fn new_player(id: PlayerId, players: HashMap<PlayerId, Player>) -> Self {
        Self {
            us: GameEntity::Player(id),
            players,
            stage: GameStage::new(),
        }
    }

    pub fn stage(&self) -> GameStage {
        self.stage
    }

    pub fn players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }

    pub fn add_player(&mut self, player: Player) {
        let _ = self.players.insert(player.player_id(), player);
    }
}
