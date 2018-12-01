use messages::KtuluMessage;

pub trait KtuluMessageHandler {
    type Endpoint;

    fn handle_message(
        &mut self,
        sender: Self::Endpoint,
        message: KtuluMessage,
    ) -> Vec<(Self::Endpoint, KtuluMessage)>;
}
