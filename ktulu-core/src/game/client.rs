use error::*;
use game_state::*;
use interface::*;
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
    ) -> Result<Vec<KtuluMessage<Endpoint>>> {
        self.game_state = Some(GameState::new_player(our_id, players));
        Ok(vec![])
    }

    fn handle_player_join(&mut self, player: Player) -> Result<Vec<KtuluMessage<Endpoint>>> {
        if let Some(ref mut gs) = self.game_state {
            gs.add_player(player);
            Ok(vec![])
        } else {
            Err(KtuluError::ClientNotInitialized)
        }
    }
}

#[cfg(test)]
impl<Endpoint: Clone> KtuluClient<Endpoint> {
    pub fn players(&self) -> Option<&HashMap<PlayerId, Player>> {
        self.game_state.as_ref().map(|gs| gs.players())
    }
}

impl<Endpoint: Clone> KtuluMessageHandler for KtuluClient<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        _sender: Endpoint,
        packet: KtuluPacket,
    ) -> Result<Vec<KtuluMessage<Endpoint>>> {
        match packet {
            KtuluPacket::Client(_) => Err(KtuluError::InvalidMessageSource),
            KtuluPacket::Server(msg) => match msg {
                ServerMsg::Accepted(AcceptedResponse { id, players }) => {
                    self.handle_accept_response(id, players)
                }
                ServerMsg::PlayerJoin(player) => self.handle_player_join(player),
                _ => Ok(vec![]),
            },
        }
    }
}
