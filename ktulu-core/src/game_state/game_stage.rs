use characters::{CharacterType, Fraction};

/// Enum representing the moment in game.
/// When the game starts, it's `BeforeStart`. Then it transitions to `Night(0)`, then `Day(1)`,
/// `Night(1)`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Time {
    BeforeStart,
    Day(u8),
    DayVoteSearch(u8),
    DayVoteKill(u8),
    Night(u8),
}

/// Enum representing an activity taking place in a given moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    /// A character uses its ability
    Character(CharacterType),
    /// A fraction wakes up
    FractionWake(Fraction),
    /// A fraction goes to sleep
    FractionSleep(Fraction),
}

const NIGHT_0_SEQUENCE: [Activity; 12] = [Activity::Character(CharacterType::Prostitute),
                                          Activity::Character(CharacterType::Pastor),
                                          Activity::Character(CharacterType::Seducer),
                                          Activity::Character(CharacterType::Sheriff),
                                          Activity::FractionWake(Fraction::Bandits),
                                          Activity::Character(CharacterType::Boss),
                                          Activity::Character(CharacterType::Blackmailer),
                                          Activity::FractionSleep(Fraction::Bandits),
                                          Activity::FractionWake(Fraction::Indians),
                                          Activity::FractionSleep(Fraction::Indians),
                                          Activity::FractionWake(Fraction::Aliens),
                                          Activity::FractionSleep(Fraction::Aliens)];

const NIGHT_SEQUENCE: [Activity; 27] = [Activity::Character(CharacterType::Sheriff),
                                        Activity::Character(CharacterType::Pastor),
                                        Activity::Character(CharacterType::TaxCollector),
                                        Activity::Character(CharacterType::Bodyguard),
                                        Activity::Character(CharacterType::Gambler),
                                        Activity::Character(CharacterType::Drunkard),
                                        Activity::FractionWake(Fraction::Bandits),
                                        Activity::Character(CharacterType::Boss),
                                        Activity::Character(CharacterType::Avenger),
                                        Activity::Character(CharacterType::Thief),
                                        Activity::Character(CharacterType::CardShark),
                                        Activity::FractionSleep(Fraction::Bandits),
                                        Activity::Character(CharacterType::WitchDoctor),
                                        Activity::FractionWake(Fraction::Indians),
                                        Activity::Character(CharacterType::Chief),
                                        Activity::Character(CharacterType::Warrior),
                                        Activity::Character(CharacterType::LoneCoyote),
                                        Activity::FractionSleep(Fraction::Indians),
                                        Activity::Character(CharacterType::SilentFoot),
                                        Activity::Character(CharacterType::ScopeEye),
                                        Activity::Character(CharacterType::Witch),
                                        Activity::FractionWake(Fraction::Aliens),
                                        Activity::Character(CharacterType::Detector),
                                        Activity::Character(CharacterType::MindEater),
                                        Activity::Character(CharacterType::GreenTentacle),
                                        Activity::Character(CharacterType::GreatAlien),
                                        Activity::FractionSleep(Fraction::Aliens)];

/// Structure representing the current state in the game - which day/night it is and what is
/// happening at the moment.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GameStage {
    time: Time,
    activity: Option<Activity>,
}

impl GameStage {
    /// Creates a new `GameStage` that describes a moment just before the start of the game
    pub fn new() -> GameStage {
        GameStage {
            time: Time::BeforeStart,
            activity: None,
        }
    }

    #[cfg(test)]
    pub fn new_with_data(time: Time, activity: Option<Activity>) -> GameStage {
        GameStage {
            time: time,
            activity: activity,
        }
    }

    /// Returns the time component (which day or night is it)
    pub fn time(&self) -> Time {
        self.time
    }

    /// Returns the current activity
    pub fn activity(&self) -> Option<Activity> {
        self.activity
    }

    /// Advances the stage to the next one.
    pub fn advance(&mut self) {
        match self.time {
            Time::BeforeStart => {
                if self.activity != None {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
                self.time = Time::Night(0);
                self.activity = Some(NIGHT_0_SEQUENCE[0]);
            }
            Time::Night(0) => {
                let activity_index = NIGHT_0_SEQUENCE.iter()
                    .position(|&x| Some(x) == self.activity);
                if let Some(activity_index) = activity_index {
                    if activity_index + 1 < NIGHT_0_SEQUENCE.len() {
                        self.activity = Some(NIGHT_0_SEQUENCE[activity_index + 1]);
                    } else {
                        self.time = Time::Day(1);
                        self.activity = None;
                    }
                } else {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
            }
            Time::Night(i) => {
                let activity_index = NIGHT_SEQUENCE.iter()
                    .position(|&x| Some(x) == self.activity);
                if let Some(activity_index) = activity_index {
                    if activity_index + 1 < NIGHT_SEQUENCE.len() {
                        self.activity = Some(NIGHT_SEQUENCE[activity_index + 1]);
                    } else {
                        self.time = Time::Day(i + 1);
                        self.activity = None;
                    }
                } else {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
            }
            Time::Day(i) => {
                if self.activity != None {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
                self.time = Time::DayVoteSearch(i);
            }
            Time::DayVoteSearch(i) => {
                if self.activity != None {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
                self.time = Time::DayVoteKill(i);
            }
            Time::DayVoteKill(i) => {
                if self.activity != None {
                    panic!("Invalid game stage: time={:?}, activity={:?}",
                           self.time,
                           self.activity);
                }
                self.time = Time::Night(i);
                self.activity = Some(NIGHT_SEQUENCE[0]);
            }
        }
    }
}
