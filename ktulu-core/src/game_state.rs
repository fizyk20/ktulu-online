use characters::CharacterType;
use interface::*;
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

struct PlayerData {
    character: CharacterType,
    awake: bool,
}

/// The struct representing the state of the game.
/// It contains the information about what moment we have in the game, what characters are there,
/// which one is active etc.
pub struct GameState<T: Character> {
    time: Time,
    active_character: Option<CharacterType>,
    players: HashMap<PlayerId, PlayerData>,
    statue_holder: PlayerId,
    player_objects: HashMap<PlayerId, T>,
}

impl<T: Character> Manitou for GameState<T> {
    fn say(who: PlayerId, what: &str) {
        // TODO
    }

    fn choose_person(sender: PlayerId, person: PlayerId) -> AbilityResponse {
        // TODO
        AbilityResponse::Todo
    }
}
