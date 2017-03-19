use game::{Character, KtuluGame, Manitou};
use interface::*;

#[test]
fn test_init_game() {
    let mut game = KtuluGame::<Manitou, _>::new();
    for _ in 0..15 {
        let player = Character::new();
        game.add_player(player);
    }
    assert!(game.can_start());
}
