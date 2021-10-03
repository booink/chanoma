use crate::chanoma::corr::{Corr, Correspondence, Item, Linear};

#[derive(Clone)]
pub struct Alphabets;

impl Corr for Alphabets {
    fn items(&self) -> Vec<Item> {
        (CAPITAL_LETTER_A
            + &CAPITAL_LETTER_B
            + &CAPITAL_LETTER_C
            + &CAPITAL_LETTER_D
            + &CAPITAL_LETTER_E
            + &CAPITAL_LETTER_F
            + &CAPITAL_LETTER_G
            + &CAPITAL_LETTER_H
            + &CAPITAL_LETTER_I
            + &CAPITAL_LETTER_J
            + &CAPITAL_LETTER_K
            + &CAPITAL_LETTER_L
            + &CAPITAL_LETTER_M
            + &CAPITAL_LETTER_N
            + &CAPITAL_LETTER_O
            + &CAPITAL_LETTER_P
            + &CAPITAL_LETTER_Q
            + &CAPITAL_LETTER_R
            + &CAPITAL_LETTER_S
            + &CAPITAL_LETTER_T
            + &CAPITAL_LETTER_U
            + &CAPITAL_LETTER_V
            + &CAPITAL_LETTER_W
            + &CAPITAL_LETTER_X
            + &CAPITAL_LETTER_Y
            + &CAPITAL_LETTER_Z
            + &SMALL_LETTER_A
            + &SMALL_LETTER_B
            + &SMALL_LETTER_C
            + &SMALL_LETTER_D
            + &SMALL_LETTER_E
            + &SMALL_LETTER_F
            + &SMALL_LETTER_G
            + &SMALL_LETTER_H
            + &SMALL_LETTER_I
            + &SMALL_LETTER_J
            + &SMALL_LETTER_K
            + &SMALL_LETTER_L
            + &SMALL_LETTER_M
            + &SMALL_LETTER_N
            + &SMALL_LETTER_O
            + &SMALL_LETTER_P
            + &SMALL_LETTER_Q
            + &SMALL_LETTER_R
            + &SMALL_LETTER_S
            + &SMALL_LETTER_T
            + &SMALL_LETTER_U
            + &SMALL_LETTER_V
            + &SMALL_LETTER_W
            + &SMALL_LETTER_X
            + &SMALL_LETTER_Y
            + &SMALL_LETTER_Z)
            .items()
    }
}

impl Alphabets {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

pub const CAPITAL_LETTER_A: Correspondence<Linear> = Linear::new("Ａ", "A").corr();
pub const CAPITAL_LETTER_B: Correspondence<Linear> = Linear::new("Ｂ", "B").corr();
pub const CAPITAL_LETTER_C: Correspondence<Linear> = Linear::new("Ｃ", "C").corr();
pub const CAPITAL_LETTER_D: Correspondence<Linear> = Linear::new("Ｄ", "D").corr();
pub const CAPITAL_LETTER_E: Correspondence<Linear> = Linear::new("Ｅ", "E").corr();
pub const CAPITAL_LETTER_F: Correspondence<Linear> = Linear::new("Ｆ", "F").corr();
pub const CAPITAL_LETTER_G: Correspondence<Linear> = Linear::new("Ｇ", "G").corr();
pub const CAPITAL_LETTER_H: Correspondence<Linear> = Linear::new("Ｈ", "H").corr();
pub const CAPITAL_LETTER_I: Correspondence<Linear> = Linear::new("Ｉ", "I").corr();
pub const CAPITAL_LETTER_J: Correspondence<Linear> = Linear::new("Ｊ", "J").corr();
pub const CAPITAL_LETTER_K: Correspondence<Linear> = Linear::new("Ｋ", "K").corr();
pub const CAPITAL_LETTER_L: Correspondence<Linear> = Linear::new("Ｌ", "L").corr();
pub const CAPITAL_LETTER_M: Correspondence<Linear> = Linear::new("Ｍ", "M").corr();
pub const CAPITAL_LETTER_N: Correspondence<Linear> = Linear::new("Ｎ", "N").corr();
pub const CAPITAL_LETTER_O: Correspondence<Linear> = Linear::new("Ｏ", "O").corr();
pub const CAPITAL_LETTER_P: Correspondence<Linear> = Linear::new("Ｐ", "P").corr();
pub const CAPITAL_LETTER_Q: Correspondence<Linear> = Linear::new("Ｑ", "Q").corr();
pub const CAPITAL_LETTER_R: Correspondence<Linear> = Linear::new("Ｒ", "R").corr();
pub const CAPITAL_LETTER_S: Correspondence<Linear> = Linear::new("Ｓ", "S").corr();
pub const CAPITAL_LETTER_T: Correspondence<Linear> = Linear::new("Ｔ", "T").corr();
pub const CAPITAL_LETTER_U: Correspondence<Linear> = Linear::new("Ｕ", "U").corr();
pub const CAPITAL_LETTER_V: Correspondence<Linear> = Linear::new("Ｖ", "V").corr();
pub const CAPITAL_LETTER_W: Correspondence<Linear> = Linear::new("Ｗ", "W").corr();
pub const CAPITAL_LETTER_X: Correspondence<Linear> = Linear::new("Ｘ", "X").corr();
pub const CAPITAL_LETTER_Y: Correspondence<Linear> = Linear::new("Ｙ", "Y").corr();
pub const CAPITAL_LETTER_Z: Correspondence<Linear> = Linear::new("Ｚ", "Z").corr();

pub const SMALL_LETTER_A: Correspondence<Linear> = Linear::new("ａ", "a").corr();
pub const SMALL_LETTER_B: Correspondence<Linear> = Linear::new("ｂ", "b").corr();
pub const SMALL_LETTER_C: Correspondence<Linear> = Linear::new("ｃ", "c").corr();
pub const SMALL_LETTER_D: Correspondence<Linear> = Linear::new("ｄ", "d").corr();
pub const SMALL_LETTER_E: Correspondence<Linear> = Linear::new("ｅ", "e").corr();
pub const SMALL_LETTER_F: Correspondence<Linear> = Linear::new("ｆ", "f").corr();
pub const SMALL_LETTER_G: Correspondence<Linear> = Linear::new("ｇ", "g").corr();
pub const SMALL_LETTER_H: Correspondence<Linear> = Linear::new("ｈ", "h").corr();
pub const SMALL_LETTER_I: Correspondence<Linear> = Linear::new("ｉ", "i").corr();
pub const SMALL_LETTER_J: Correspondence<Linear> = Linear::new("ｊ", "j").corr();
pub const SMALL_LETTER_K: Correspondence<Linear> = Linear::new("ｋ", "k").corr();
pub const SMALL_LETTER_L: Correspondence<Linear> = Linear::new("ｌ", "l").corr();
pub const SMALL_LETTER_M: Correspondence<Linear> = Linear::new("ｍ", "m").corr();
pub const SMALL_LETTER_N: Correspondence<Linear> = Linear::new("ｎ", "n").corr();
pub const SMALL_LETTER_O: Correspondence<Linear> = Linear::new("ｏ", "o").corr();
pub const SMALL_LETTER_P: Correspondence<Linear> = Linear::new("ｐ", "p").corr();
pub const SMALL_LETTER_Q: Correspondence<Linear> = Linear::new("ｑ", "q").corr();
pub const SMALL_LETTER_R: Correspondence<Linear> = Linear::new("ｒ", "r").corr();
pub const SMALL_LETTER_S: Correspondence<Linear> = Linear::new("ｓ", "s").corr();
pub const SMALL_LETTER_T: Correspondence<Linear> = Linear::new("ｔ", "t").corr();
pub const SMALL_LETTER_U: Correspondence<Linear> = Linear::new("ｕ", "u").corr();
pub const SMALL_LETTER_V: Correspondence<Linear> = Linear::new("ｖ", "v").corr();
pub const SMALL_LETTER_W: Correspondence<Linear> = Linear::new("ｗ", "w").corr();
pub const SMALL_LETTER_X: Correspondence<Linear> = Linear::new("ｘ", "x").corr();
pub const SMALL_LETTER_Y: Correspondence<Linear> = Linear::new("ｙ", "y").corr();
pub const SMALL_LETTER_Z: Correspondence<Linear> = Linear::new("ｚ", "z").corr();

pub const ALPHABETS: Correspondence<Alphabets> = Alphabets::new().corr();
