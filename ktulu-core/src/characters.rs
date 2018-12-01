#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterType {
    // The citizens
    Sheriff,
    Mayor,
    Prostitute,
    Pastor,
    GoodGunfighter,
    Drunkard,
    Bodyguard,
    TaxCollector,
    Doctor,
    Gambler,
    InsuranceAgent,
    Judge,
    Seducer,

    // The bandits
    Boss,
    Avenger,
    Thief,
    EvilGunfighter,
    Blackmailer,
    CardShark,

    // The Indians
    Chief,
    WitchDoctor,
    Witch,
    LoneCoyote,
    Warrior,
    ScopeEye,
    SilentFoot,

    // The aliens
    GreatAlien,
    GreenTentacle,
    Detector,
    MindEater,
}

impl CharacterType {
    pub fn fraction(&self) -> Fraction {
        use self::CharacterType::*;
        match *self {
            Sheriff | Mayor | Prostitute | Pastor | GoodGunfighter | Drunkard | Bodyguard
            | TaxCollector | Doctor | Gambler | InsuranceAgent | Judge | Seducer => {
                Fraction::Citizens
            }
            Boss | Avenger | Thief | EvilGunfighter | Blackmailer | CardShark => Fraction::Bandits,
            Chief | WitchDoctor | Witch | LoneCoyote | Warrior | ScopeEye | SilentFoot => {
                Fraction::Indians
            }
            GreatAlien | GreenTentacle | Detector | MindEater => Fraction::Aliens,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Citizens,
    Bandits,
    Indians,
    Aliens,
}

impl Fraction {
    pub fn characters(&self) -> impl Iterator<Item = &'static CharacterType> {
        use self::CharacterType::*;
        use self::Fraction::*;

        match *self {
            Citizens => [
                Sheriff,
                Mayor,
                Prostitute,
                Pastor,
                GoodGunfighter,
                Drunkard,
                Bodyguard,
                TaxCollector,
                Doctor,
                Gambler,
                InsuranceAgent,
                Judge,
                Seducer,
            ]
                .into_iter(),
            Bandits => [Boss, Avenger, Thief, EvilGunfighter, Blackmailer, CardShark].into_iter(),
            Indians => [
                Chief,
                WitchDoctor,
                Witch,
                LoneCoyote,
                Warrior,
                ScopeEye,
                SilentFoot,
            ]
                .into_iter(),
            Aliens => [GreatAlien, GreenTentacle, Detector, MindEater].into_iter(),
        }
    }
}

#[test]
fn test_fractions() {
    for fraction in &[
        Fraction::Citizens,
        Fraction::Indians,
        Fraction::Bandits,
        Fraction::Aliens,
    ] {
        assert!(fraction.characters().all(|ch| ch.fraction() == *fraction));
    }
}
