use PlayerId;
use characters::CharacterType;
use interface::CharacterClient;
use std::collections::HashMap;

const MIN_PLAYERS: usize = 12;

pub struct Manitou<Character: CharacterClient> {
    character_types: HashMap<PlayerId, CharacterType>,
    characters: HashMap<PlayerId, Character>,
    statue_holder: PlayerId,
    next_id: PlayerId,
}

impl<Character: CharacterClient> Manitou<Character> {
    pub fn new() -> Self {
        Manitou {
            character_types: HashMap::new(),
            characters: HashMap::new(),
            statue_holder: 0,
            next_id: 0,
        }
    }

    pub fn connect(&mut self, character: Character) -> PlayerId {
        self.characters.insert(self.next_id, character);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn can_start(&self) -> bool {
        self.characters.len() >= MIN_PLAYERS
    }
}
