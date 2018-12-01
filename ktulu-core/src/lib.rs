extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod characters;
pub mod game;
pub mod game_state;
pub mod interface;
pub mod messages;
#[cfg(test)]
mod tests;

pub type PlayerId = u8;
