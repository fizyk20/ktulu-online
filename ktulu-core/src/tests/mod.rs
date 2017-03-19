mod mock;
use self::mock::*;

#[test]
fn test_game_flow() {
    let env = TestEnvironment::new();
    for _ in 0..15 {
        env.add_player();
    }
    assert!(env.borrow().manitou().can_start());
}
