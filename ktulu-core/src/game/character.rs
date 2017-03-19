use PlayerId;
use characters::CharacterType;
use interface::ManitouClient;

pub struct Character<Manitou: ManitouClient> {
    manitou: Manitou,
    my_id: PlayerId,
    my_type: Option<CharacterType>,
    awake: bool,
}

impl<Manitou: ManitouClient> Character<Manitou> {
    pub fn new(mut manitou: Manitou) -> Self {
        let my_id = manitou.connect();
        Character {
            manitou: manitou,
            my_id: my_id,
            my_type: None,
            awake: false,
        }
    }
}
