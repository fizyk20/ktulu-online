use characters::{CharacterType, Fraction};
use game_state::{Activity, GameStage, Time};

#[test]
fn test_initial_game_stages() {
    let mut stage = GameStage::new();
    assert_eq!(stage.time(), Time::BeforeStart);
    assert_eq!(stage.activity(), None);
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::Prostitute)));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::Pastor)));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::Seducer)));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(0));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::Sheriff)));
}

#[test]
fn test_end_night0() {
    let mut stage = GameStage::new_with_data(Time::Night(0),
                                             Some(Activity::FractionSleep(Fraction::Aliens)));
    stage.advance();
    assert_eq!(stage.time(), Time::Day(1));
    assert_eq!(stage.activity(), None);
}

#[test]
fn test_end_night() {
    let mut stage =
        GameStage::new_with_data(Time::Night(2),
                                 Some(Activity::Character(CharacterType::GreenTentacle)));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(2));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::GreatAlien)));
    stage.advance();
    assert_eq!(stage.time(), Time::Night(2));
    assert_eq!(stage.activity(),
               Some(Activity::FractionSleep(Fraction::Aliens)));
    stage.advance();
    assert_eq!(stage.time(), Time::Day(3));
    assert_eq!(stage.activity(), None);
}

#[test]
fn test_end_day() {
    let mut stage = GameStage::new_with_data(Time::Day(3), None);
    stage.advance();
    assert_eq!(stage.time(), Time::DayVoteSearch(3));
    assert_eq!(stage.activity(), None);
    stage.advance();
    assert_eq!(stage.time(), Time::DayVoteKill(3));
    assert_eq!(stage.activity(), None);
    stage.advance();
    assert_eq!(stage.time(), Time::Night(3));
    assert_eq!(stage.activity(),
               Some(Activity::Character(CharacterType::Sheriff)));
}

#[test]
#[should_panic]
fn test_invalid_stage_night() {
    let mut stage = GameStage::new_with_data(Time::Night(1), None);
    stage.advance();
}

#[test]
#[should_panic]
fn test_invalid_stage_day() {
    let mut stage = GameStage::new_with_data(Time::Day(1),
                                             Some(Activity::FractionWake(Fraction::Citizens)));
    stage.advance();
}
