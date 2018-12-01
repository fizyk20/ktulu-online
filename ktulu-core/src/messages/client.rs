#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMsg {
    Connect { nick: String },
}
