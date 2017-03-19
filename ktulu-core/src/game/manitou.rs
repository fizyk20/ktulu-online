use PlayerId;
use characters::CharacterType;
use interface::ManitouTrait;
use std::collections::HashMap;

pub struct Manitou {
    characters: HashMap<PlayerId, CharacterType>,
    statue_holder: PlayerId,
}

impl ManitouTrait for Manitou {
    fn new() -> Self {
        Manitou {
            characters: HashMap::new(),
            statue_holder: 0,
        }
    }
}
