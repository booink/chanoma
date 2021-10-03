use crate::chanoma::corr::{Corr, Correspondence, Item, Linear};

#[derive(Clone)]
pub struct Digits;

impl Corr for Digits {
    fn items(&self) -> Vec<Item> {
        (DIGIT_ZERO
            + &DIGIT_ONE
            + &DIGIT_TWO
            + &DIGIT_THREE
            + &DIGIT_FOUR
            + &DIGIT_FIVE
            + &DIGIT_SIX
            + &DIGIT_SEVEN
            + &DIGIT_EIGHT
            + &DIGIT_NINE)
            .items()
    }
}

impl Digits {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

pub const DIGIT_ZERO: Correspondence<Linear> = Linear::new("０", "0").corr();
pub const DIGIT_ONE: Correspondence<Linear> = Linear::new("１", "1").corr();
pub const DIGIT_TWO: Correspondence<Linear> = Linear::new("２", "2").corr();
pub const DIGIT_THREE: Correspondence<Linear> = Linear::new("３", "3").corr();
pub const DIGIT_FOUR: Correspondence<Linear> = Linear::new("４", "4").corr();
pub const DIGIT_FIVE: Correspondence<Linear> = Linear::new("５", "5").corr();
pub const DIGIT_SIX: Correspondence<Linear> = Linear::new("６", "6").corr();
pub const DIGIT_SEVEN: Correspondence<Linear> = Linear::new("７", "7").corr();
pub const DIGIT_EIGHT: Correspondence<Linear> = Linear::new("８", "8").corr();
pub const DIGIT_NINE: Correspondence<Linear> = Linear::new("９", "9").corr();

pub const DIGITS: Correspondence<Digits> = Digits::new().corr();
