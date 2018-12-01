use messages::{KtuluMessage, KtuluPacket};

pub trait KtuluMessageHandler {
    type Endpoint;

    fn handle_message(
        &mut self,
        sender: Self::Endpoint,
        packet: KtuluPacket,
    ) -> Vec<KtuluMessage<Self::Endpoint>>;
}
