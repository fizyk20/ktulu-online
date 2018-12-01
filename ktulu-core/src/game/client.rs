use game_state::GameState;
use interface::KtuluMessageHandler;
use messages::{ClientMsg, KtuluMessage, KtuluPacket};

pub struct KtuluClient<Endpoint: Clone> {
    manitou: Endpoint,
    our_nick: String,
    game_state: Option<GameState>,
}

impl<Endpoint: Clone> KtuluClient<Endpoint> {
    pub fn new(nick: String, manitou: Endpoint) -> Self {
        Self {
            manitou,
            our_nick: nick,
            game_state: None,
        }
    }

    pub fn connect(&self) -> KtuluMessage<Endpoint> {
        KtuluMessage {
            recipient: self.manitou.clone(),
            packet: KtuluPacket::Client(ClientMsg::Connect {
                nick: self.our_nick.clone(),
            }),
        }
    }
}

impl<Endpoint: Clone> KtuluMessageHandler for KtuluClient<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        sender: Endpoint,
        packet: KtuluPacket,
    ) -> Vec<KtuluMessage<Endpoint>> {
        vec![]
    }
}
