mod client;
mod server;

pub use self::client::*;
pub use self::server::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KtuluPacket {
    Client(ClientMsg),
    Server(ServerMsg),
}

pub struct KtuluMessage<Endpoint> {
    pub recipient: Endpoint,
    pub packet: KtuluPacket,
}
