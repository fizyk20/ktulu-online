use game_state::GameState;
use interface::KtuluMessageHandler;
use messages::KtuluMessage;
use std::collections::HashMap;
use std::hash::Hash;
use PlayerId;

pub struct KtuluServer<Endpoint: Hash + Eq> {
    id_endpoint_map: HashMap<PlayerId, Endpoint>,
    endpoint_id_map: HashMap<Endpoint, PlayerId>,
    game_state: GameState,
}

impl<Endpoint: Hash + Eq> KtuluServer<Endpoint> {
    pub fn new() -> Self {
        Self {
            id_endpoint_map: HashMap::new(),
            endpoint_id_map: HashMap::new(),
            game_state: GameState::new_manitou(),
        }
    }
}

impl<Endpoint: Hash + Eq> KtuluMessageHandler for KtuluServer<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        sender: Endpoint,
        message: KtuluMessage,
    ) -> Vec<(Endpoint, KtuluMessage)> {
        vec![]
    }
}
