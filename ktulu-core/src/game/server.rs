use error::*;
use game_state::*;
use interface::*;
use messages::*;
use std::collections::HashMap;
use std::hash::Hash;
use PlayerId;

const MIN_PLAYERS: usize = 13;
const MAX_PLAYERS: usize = 30;

pub struct KtuluServer<Endpoint: Hash + Eq + Clone> {
    id_endpoint_map: HashMap<PlayerId, Endpoint>,
    endpoint_id_map: HashMap<Endpoint, PlayerId>,
    game_state: GameState,
}

impl<Endpoint: Hash + Eq + Clone> KtuluServer<Endpoint> {
    pub fn new() -> Self {
        Self {
            id_endpoint_map: HashMap::new(),
            endpoint_id_map: HashMap::new(),
            game_state: GameState::new_manitou(),
        }
    }

    pub fn can_start(&self) -> bool {
        self.id_endpoint_map.len() >= MIN_PLAYERS
    }

    pub fn handle_connect(
        &mut self,
        sender: Endpoint,
        nick: String,
    ) -> Result<Vec<KtuluMessage<Endpoint>>> {
        if self.game_state.stage().time() != Time::BeforeStart {
            let response = KtuluMessage {
                recipient: sender,
                packet: KtuluPacket::Server(ServerMsg::Rejected(RejectionReason::GameInProgress)),
            };
            return Ok(vec![response]);
        }
        if self.id_endpoint_map.len() >= MAX_PLAYERS {
            let response = KtuluMessage {
                recipient: sender,
                packet: KtuluPacket::Server(ServerMsg::Rejected(RejectionReason::ServerFull)),
            };
            return Ok(vec![response]);
        }
        let new_id = self.id_endpoint_map.len() as PlayerId;
        let _ = self.id_endpoint_map.insert(new_id, sender.clone());
        let _ = self.endpoint_id_map.insert(sender.clone(), new_id);
        let new_player = Player::new(new_id, nick);
        self.game_state.add_player(new_player.clone());

        // respond with acceptance
        let mut output_msgs = vec![KtuluMessage {
            recipient: sender,
            packet: KtuluPacket::Server(ServerMsg::Accepted(AcceptedResponse {
                id: new_id,
                players: self.game_state.players().clone(),
            })),
        }];

        for (_, endpoint) in self.id_endpoint_map.iter().filter(|&(&id, _)| id != new_id) {
            let msg = KtuluMessage {
                recipient: endpoint.clone(),
                packet: KtuluPacket::Server(ServerMsg::PlayerJoin(new_player.clone())),
            };
            output_msgs.push(msg);
        }

        Ok(output_msgs)
    }
}

impl<Endpoint: Hash + Eq + Clone> KtuluMessageHandler for KtuluServer<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        sender: Endpoint,
        packet: KtuluPacket,
    ) -> Result<Vec<KtuluMessage<Endpoint>>> {
        match packet {
            KtuluPacket::Server(_) => Err(KtuluError::InvalidMessageSource),
            KtuluPacket::Client(msg) => match msg {
                ClientMsg::Connect { nick } => self.handle_connect(sender, nick),
                _ => Ok(vec![]),
            },
        }
    }
}
