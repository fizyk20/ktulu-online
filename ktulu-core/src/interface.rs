/// This will be a trait implemented by all character structs and by their proxies
pub trait Character {
    fn go_to_sleep();
    fn wake_up();
}

/// This will be a trait implemented by GameState and GameStateProxy
pub trait Manitou {
    fn end_day();
    fn end_night();
}
