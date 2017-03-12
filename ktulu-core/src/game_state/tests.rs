use characters::CharacterType;
use game_state::{GameStage, Time};

#[test]
fn test_game_stages() {
    let mut stage = GameStage::new();
    assert_eq!(stage.time(), Time::BeforeStart);
    assert_eq!(stage.active_character(), None);
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.active_character(), Some(CharacterType::Prostitute));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.active_character(), Some(CharacterType::Pastor));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.active_character(), Some(CharacterType::Sheriff));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.active_character(), Some(CharacterType::Boss));
}
