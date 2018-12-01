use game_state::Player;
use std::collections::HashMap;
use PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMsg {
    Rejected(RejectionReason),
    Accepted(AcceptedResponse),
    PlayerJoin(Player),
    PlayerLeave(PlayerId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RejectionReason {
    GameInProgress,
    ServerFull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedResponse {
    pub id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
}
