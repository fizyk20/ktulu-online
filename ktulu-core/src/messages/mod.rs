mod client;
mod server;

pub use self::client::*;
pub use self::server::*;

#[derive(Debug, Clone)]
pub enum KtuluMessage {
    Client(ClientMsg),
    Server(ServerMsg),
}
