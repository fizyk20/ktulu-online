use PlayerId;

/// This is a trait for objects that will allow Manitou to perform RPCs on the characters.
pub trait CharacterClient {}

/// This is a trait for objects that will allow characters to perform RPCs on the Manitou.
pub trait ManitouClient {
    fn connect(&mut self) -> PlayerId;
}
