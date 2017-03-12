use characters::CharacterType;

/// Enum representing the moment in game.
/// When the game starts, it's `BeforeStart`. Then it transitions to `Night(0)`, then `Day(1)`,
/// `Night(1)`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Time {
    BeforeStart,
    Day(u8),
    Night(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GameStage {
    time: Time,
    active_character: Option<CharacterType>,
}

impl GameStage {
    pub fn new() -> GameStage {
        GameStage {
            time: Time::BeforeStart,
            active_character: None,
        }
    }

    pub fn time(&self) -> Time {
        self.time
    }

    pub fn active_character(&self) -> Option<CharacterType> {
        self.active_character
    }

    pub fn advance(&mut self) {
        match (self.time, self.active_character) {
            (Time::BeforeStart, None) => {
                self.time = Time::Night(0);
                self.active_character = Some(CharacterType::Prostitute);
            }
            (Time::Night(0), Some(CharacterType::Prostitute)) => {
                self.active_character = Some(CharacterType::Pastor);
            }
            (Time::Night(0), Some(CharacterType::Pastor)) => {
                self.active_character = Some(CharacterType::Sheriff);
            }
            (Time::Night(0), Some(CharacterType::Sheriff)) => {
                self.active_character = Some(CharacterType::Boss);
            }
            _ => {
                panic!("Invalid game stage: (time={:?}, active_character={:?})",
                       self.time,
                       self.active_character)
            }
        }
    }
}
