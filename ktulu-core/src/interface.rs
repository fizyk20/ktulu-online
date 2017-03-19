/// This will be a trait implemented by all character structs and by their proxies
pub trait CharacterTrait {
    fn new() -> Self;
}

/// This will be a trait implemented by GameState and GameStateProxy
pub trait ManitouTrait {
    fn new() -> Self;
}
