use PlayerId;
use game_state::GameStage;
use interface::*;
use std::collections::HashMap;

const MIN_PLAYERS: usize = 12;

pub struct KtuluGame<Manitou: ManitouTrait, Character: CharacterTrait> {
    manitou: Manitou,
    players: HashMap<PlayerId, Character>,
    free_id: PlayerId,
    stage: GameStage,
}

impl<Manitou: ManitouTrait, Character: CharacterTrait> KtuluGame<Manitou, Character> {
    pub fn new() -> KtuluGame<Manitou, Character> {
        KtuluGame {
            manitou: Manitou::new(),
            players: HashMap::new(),
            free_id: 0,
            stage: GameStage::new(),
        }
    }

    pub fn add_player(&mut self, player: Character) -> PlayerId {
        self.players.insert(self.free_id, player);
        self.free_id += 1;
        self.free_id - 1
    }

    pub fn can_start(&self) -> bool {
        self.players.len() >= MIN_PLAYERS
    }
}
