use error::KtuluError;
use messages::{KtuluMessage, KtuluPacket};

pub type Result<T> = ::std::result::Result<T, KtuluError>;

pub trait KtuluMessageHandler {
    type Endpoint;

    fn handle_message(
        &mut self,
        sender: Self::Endpoint,
        packet: KtuluPacket,
    ) -> Result<Vec<KtuluMessage<Self::Endpoint>>>;
}
