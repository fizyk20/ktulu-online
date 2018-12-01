mod mock;
use self::mock::*;
use messages::KtuluMessage;

#[test]
fn test_game_flow() {
    let mut env = TestEnvironment::new();
    for nick in &NAMES[..15] {
        env.add_player(nick.to_string());
    }

    let connect_msgs: Vec<_> = env
        .characters()
        .map(|(sender, ch)| (sender, ch.connect()))
        .collect();
    for (sender, KtuluMessage { recipient, packet }) in connect_msgs {
        env.send_message(sender, recipient, packet);
    }

    env.handle_all_messages();

    assert!(env.manitou().can_start());
}
