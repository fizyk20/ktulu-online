use characters::{CharacterType, Fraction};
use PlayerId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HasStatue {
    Yes,
    No,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AwakeState {
    Awake,
    Sleeping,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: PlayerId,
    nick: String,
    pub has_statue: HasStatue,
    pub awake: AwakeState,
    character: Option<CharacterType>,
}

impl Player {
    pub fn new(id: PlayerId, nick: String) -> Self {
        Self {
            id,
            nick,
            has_statue: HasStatue::Unknown,
            awake: AwakeState::Unknown,
            character: None,
        }
    }

    pub fn set_character(&mut self, character: CharacterType) {
        self.character = Some(character);
    }

    pub fn player_id(&self) -> PlayerId {
        self.id
    }

    pub fn character(&self) -> Option<CharacterType> {
        self.character
    }

    pub fn fraction(&self) -> Option<Fraction> {
        self.character.map(|ch| ch.fraction())
    }
}
