use characters::CharacterType;
use interface::CharacterTrait;

pub struct Character {
    my_type: Option<CharacterType>,
    awake: bool,
}

impl CharacterTrait for Character {
    fn new() -> Self {
        Character {
            my_type: None,
            awake: false,
        }
    }
}
