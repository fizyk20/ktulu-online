use game_state::PlayerId;

/// This will be a trait implemented by all character structs and by their proxies
pub trait Character {
    fn go_to_sleep();
    fn wake_up();
    fn ask_choice();
    fn ask_vote();
    fn handle_say(who: PlayerId, what: &str);
}

pub enum AbilityResponse {
    Todo,
}

/// This will be a trait implemented by GameState and GameStateProxy
pub trait Manitou {
    fn say(sender: PlayerId, what: &str);
    fn choose_person(sender: PlayerId, person: PlayerId) -> AbilityResponse;
}
