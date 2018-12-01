use game_state::*;
use interface::KtuluMessageHandler;
use messages::*;
use std::collections::HashMap;
use PlayerId;

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

    fn handle_accept_response(
        &mut self,
        our_id: PlayerId,
        players: HashMap<PlayerId, Player>,
    ) -> Vec<KtuluMessage<Endpoint>> {
        self.game_state = Some(GameState::new_player(our_id, players));
        vec![]
    }
}

impl<Endpoint: Clone> KtuluMessageHandler for KtuluClient<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        sender: Endpoint,
        packet: KtuluPacket,
    ) -> Vec<KtuluMessage<Endpoint>> {
        match packet {
            KtuluPacket::Client(_) => {
                // TODO log some kind of error
                vec![]
            }
            KtuluPacket::Server(msg) => match msg {
                ServerMsg::Accepted(AcceptedResponse { id, players }) => {
                    self.handle_accept_response(id, players)
                }
                _ => vec![],
            },
        }
    }
}
