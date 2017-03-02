use characters::CharacterType;
use std::collections::HashMap;

/// Enum representing the moment in game.
/// When the game starts, it's `BeforeStart`. Then it transitions to `Night(0)`, then `Day(1)`,
/// `Night(1)`, etc.
pub enum Time {
    BeforeStart,
    Day(u8),
    Night(u8),
}

pub type PlayerId = u8;

/// The struct representing the state of the game.
/// It contains the information about what moment we have in the game, what characters are there,
/// which one is active etc.
pub struct GameState {
    time: Time,
    active_character: Option<CharacterType>,
    players: HashMap<PlayerId, CharacterType>,
    statue_holder: PlayerId,
}
