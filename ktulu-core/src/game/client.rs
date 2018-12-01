use game_state::GameState;
use interface::KtuluMessageHandler;
use messages::KtuluMessage;

pub struct KtuluClient<Endpoint> {
    manitou: Endpoint,
    our_nick: String,
    game_state: Option<GameState>,
}

impl<Endpoint> KtuluClient<Endpoint> {
    pub fn new(nick: String, manitou: Endpoint) -> Self {
        Self {
            manitou,
            our_nick: nick,
            game_state: None,
        }
    }
}

impl<Endpoint> KtuluMessageHandler for KtuluClient<Endpoint> {
    type Endpoint = Endpoint;

    fn handle_message(
        &mut self,
        sender: Endpoint,
        message: KtuluMessage,
    ) -> Vec<(Endpoint, KtuluMessage)> {
        vec![]
    }
}
