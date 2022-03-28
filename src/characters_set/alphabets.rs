//! 全角アルファベットから半角アルファベットの置換テーブルを定義しているモジュールです。
use crate::corr::{Corr, Correspondence, Item, Linear};

/// 全角アルファベットから半角アルファベットの置換テーブルを保持する構造体です。
#[derive(Clone)]
pub struct Alphabets;

impl Corr for Alphabets {
    fn items(&self) -> Vec<Item> {
        (Self::CAPITAL_LETTER_A
            + &Self::CAPITAL_LETTER_B
            + &Self::CAPITAL_LETTER_C
            + &Self::CAPITAL_LETTER_D
            + &Self::CAPITAL_LETTER_E
            + &Self::CAPITAL_LETTER_F
            + &Self::CAPITAL_LETTER_G
            + &Self::CAPITAL_LETTER_H
            + &Self::CAPITAL_LETTER_I
            + &Self::CAPITAL_LETTER_J
            + &Self::CAPITAL_LETTER_K
            + &Self::CAPITAL_LETTER_L
            + &Self::CAPITAL_LETTER_M
            + &Self::CAPITAL_LETTER_N
            + &Self::CAPITAL_LETTER_O
            + &Self::CAPITAL_LETTER_P
            + &Self::CAPITAL_LETTER_Q
            + &Self::CAPITAL_LETTER_R
            + &Self::CAPITAL_LETTER_S
            + &Self::CAPITAL_LETTER_T
            + &Self::CAPITAL_LETTER_U
            + &Self::CAPITAL_LETTER_V
            + &Self::CAPITAL_LETTER_W
            + &Self::CAPITAL_LETTER_X
            + &Self::CAPITAL_LETTER_Y
            + &Self::CAPITAL_LETTER_Z
            + &Self::SMALL_LETTER_A
            + &Self::SMALL_LETTER_B
            + &Self::SMALL_LETTER_C
            + &Self::SMALL_LETTER_D
            + &Self::SMALL_LETTER_E
            + &Self::SMALL_LETTER_F
            + &Self::SMALL_LETTER_G
            + &Self::SMALL_LETTER_H
            + &Self::SMALL_LETTER_I
            + &Self::SMALL_LETTER_J
            + &Self::SMALL_LETTER_K
            + &Self::SMALL_LETTER_L
            + &Self::SMALL_LETTER_M
            + &Self::SMALL_LETTER_N
            + &Self::SMALL_LETTER_O
            + &Self::SMALL_LETTER_P
            + &Self::SMALL_LETTER_Q
            + &Self::SMALL_LETTER_R
            + &Self::SMALL_LETTER_S
            + &Self::SMALL_LETTER_T
            + &Self::SMALL_LETTER_U
            + &Self::SMALL_LETTER_V
            + &Self::SMALL_LETTER_W
            + &Self::SMALL_LETTER_X
            + &Self::SMALL_LETTER_Y
            + &Self::SMALL_LETTER_Z)
            .items()
    }
}

impl Alphabets {
    /// Ａ (全角大文字) -> A (半角大文字) の置換です。
    pub const CAPITAL_LETTER_A: Correspondence<Linear> = Linear::new("Ａ", "A").corr();
    /// Ｂ (全角大文字) -> B (半角大文字) の置換です。
    pub const CAPITAL_LETTER_B: Correspondence<Linear> = Linear::new("Ｂ", "B").corr();
    /// Ｃ (全角大文字) -> C (半角大文字) の置換です。
    pub const CAPITAL_LETTER_C: Correspondence<Linear> = Linear::new("Ｃ", "C").corr();
    /// Ｄ (全角大文字) -> D (半角大文字) の置換です。
    pub const CAPITAL_LETTER_D: Correspondence<Linear> = Linear::new("Ｄ", "D").corr();
    /// Ｅ (全角大文字) -> E (半角大文字) の置換です。
    pub const CAPITAL_LETTER_E: Correspondence<Linear> = Linear::new("Ｅ", "E").corr();
    /// Ｆ (全角大文字) -> F (半角大文字) の置換です。
    pub const CAPITAL_LETTER_F: Correspondence<Linear> = Linear::new("Ｆ", "F").corr();
    /// Ｇ (全角大文字) -> G (半角大文字) の置換です。
    pub const CAPITAL_LETTER_G: Correspondence<Linear> = Linear::new("Ｇ", "G").corr();
    /// Ｈ (全角大文字) -> H (半角大文字) の置換です。
    pub const CAPITAL_LETTER_H: Correspondence<Linear> = Linear::new("Ｈ", "H").corr();
    /// Ｉ (全角大文字) -> I (半角大文字) の置換です。
    pub const CAPITAL_LETTER_I: Correspondence<Linear> = Linear::new("Ｉ", "I").corr();
    /// Ｊ (全角大文字) -> J (半角大文字) の置換です。
    pub const CAPITAL_LETTER_J: Correspondence<Linear> = Linear::new("Ｊ", "J").corr();
    /// Ｋ (全角大文字) -> K (半角大文字) の置換です。
    pub const CAPITAL_LETTER_K: Correspondence<Linear> = Linear::new("Ｋ", "K").corr();
    /// Ｌ (全角大文字) -> L (半角大文字) の置換です。
    pub const CAPITAL_LETTER_L: Correspondence<Linear> = Linear::new("Ｌ", "L").corr();
    /// Ｍ (全角大文字) -> M (半角大文字) の置換です。
    pub const CAPITAL_LETTER_M: Correspondence<Linear> = Linear::new("Ｍ", "M").corr();
    /// Ｎ (全角大文字) -> N (半角大文字) の置換です。
    pub const CAPITAL_LETTER_N: Correspondence<Linear> = Linear::new("Ｎ", "N").corr();
    /// Ｏ (全角大文字) -> O (半角大文字) の置換です。
    pub const CAPITAL_LETTER_O: Correspondence<Linear> = Linear::new("Ｏ", "O").corr();
    /// Ｐ (全角大文字) -> P (半角大文字) の置換です。
    pub const CAPITAL_LETTER_P: Correspondence<Linear> = Linear::new("Ｐ", "P").corr();
    /// Ｑ (全角大文字) -> Q (半角大文字) の置換です。
    pub const CAPITAL_LETTER_Q: Correspondence<Linear> = Linear::new("Ｑ", "Q").corr();
    /// Ｒ (全角大文字) -> R (半角大文字) の置換です。
    pub const CAPITAL_LETTER_R: Correspondence<Linear> = Linear::new("Ｒ", "R").corr();
    /// Ｓ (全角大文字) -> S (半角大文字) の置換です。
    pub const CAPITAL_LETTER_S: Correspondence<Linear> = Linear::new("Ｓ", "S").corr();
    /// Ｔ (全角大文字) -> T (半角大文字) の置換です。
    pub const CAPITAL_LETTER_T: Correspondence<Linear> = Linear::new("Ｔ", "T").corr();
    /// Ｕ (全角大文字) -> U (半角大文字) の置換です。
    pub const CAPITAL_LETTER_U: Correspondence<Linear> = Linear::new("Ｕ", "U").corr();
    /// Ｖ (全角大文字) -> V (半角大文字) の置換です。
    pub const CAPITAL_LETTER_V: Correspondence<Linear> = Linear::new("Ｖ", "V").corr();
    /// Ｗ (全角大文字) -> W (半角大文字) の置換です。
    pub const CAPITAL_LETTER_W: Correspondence<Linear> = Linear::new("Ｗ", "W").corr();
    /// Ｘ (全角大文字) -> X (半角大文字) の置換です。
    pub const CAPITAL_LETTER_X: Correspondence<Linear> = Linear::new("Ｘ", "X").corr();
    /// Ｙ (全角大文字) -> Y (半角大文字) の置換です。
    pub const CAPITAL_LETTER_Y: Correspondence<Linear> = Linear::new("Ｙ", "Y").corr();
    /// Ｚ (全角大文字) -> Z (半角大文字) の置換です。
    pub const CAPITAL_LETTER_Z: Correspondence<Linear> = Linear::new("Ｚ", "Z").corr();

    /// ａ (全角小文字) -> a (半角小文字) の置換です。
    pub const SMALL_LETTER_A: Correspondence<Linear> = Linear::new("ａ", "a").corr();
    /// ｂ (全角小文字) -> b (半角小文字) の置換です。
    pub const SMALL_LETTER_B: Correspondence<Linear> = Linear::new("ｂ", "b").corr();
    /// ｃ (全角小文字) -> c (半角小文字) の置換です。
    pub const SMALL_LETTER_C: Correspondence<Linear> = Linear::new("ｃ", "c").corr();
    /// ｄ (全角小文字) -> d (半角小文字) の置換です。
    pub const SMALL_LETTER_D: Correspondence<Linear> = Linear::new("ｄ", "d").corr();
    /// ｅ (全角小文字) -> e (半角小文字) の置換です。
    pub const SMALL_LETTER_E: Correspondence<Linear> = Linear::new("ｅ", "e").corr();
    /// ｆ (全角小文字) -> f (半角小文字) の置換です。
    pub const SMALL_LETTER_F: Correspondence<Linear> = Linear::new("ｆ", "f").corr();
    /// ｇ (全角小文字) -> g (半角小文字) の置換です。
    pub const SMALL_LETTER_G: Correspondence<Linear> = Linear::new("ｇ", "g").corr();
    /// ｈ (全角小文字) -> h (半角小文字) の置換です。
    pub const SMALL_LETTER_H: Correspondence<Linear> = Linear::new("ｈ", "h").corr();
    /// ｉ (全角小文字) -> i (半角小文字) の置換です。
    pub const SMALL_LETTER_I: Correspondence<Linear> = Linear::new("ｉ", "i").corr();
    /// ｊ (全角小文字) -> j (半角小文字) の置換です。
    pub const SMALL_LETTER_J: Correspondence<Linear> = Linear::new("ｊ", "j").corr();
    /// ｋ (全角小文字) -> k (半角小文字) の置換です。
    pub const SMALL_LETTER_K: Correspondence<Linear> = Linear::new("ｋ", "k").corr();
    /// ｌ (全角小文字) -> l (半角小文字) の置換です。
    pub const SMALL_LETTER_L: Correspondence<Linear> = Linear::new("ｌ", "l").corr();
    /// ｍ (全角小文字) -> m (半角小文字) の置換です。
    pub const SMALL_LETTER_M: Correspondence<Linear> = Linear::new("ｍ", "m").corr();
    /// ｎ (全角小文字) -> n (半角小文字) の置換です。
    pub const SMALL_LETTER_N: Correspondence<Linear> = Linear::new("ｎ", "n").corr();
    /// ｏ (全角小文字) -> o (半角小文字) の置換です。
    pub const SMALL_LETTER_O: Correspondence<Linear> = Linear::new("ｏ", "o").corr();
    /// ｐ (全角小文字) -> p (半角小文字) の置換です。
    pub const SMALL_LETTER_P: Correspondence<Linear> = Linear::new("ｐ", "p").corr();
    /// ｑ (全角小文字) -> q (半角小文字) の置換です。
    pub const SMALL_LETTER_Q: Correspondence<Linear> = Linear::new("ｑ", "q").corr();
    /// ｒ (全角小文字) -> r (半角小文字) の置換です。
    pub const SMALL_LETTER_R: Correspondence<Linear> = Linear::new("ｒ", "r").corr();
    /// ｓ (全角小文字) -> s (半角小文字) の置換です。
    pub const SMALL_LETTER_S: Correspondence<Linear> = Linear::new("ｓ", "s").corr();
    /// ｔ (全角小文字) -> t (半角小文字) の置換です。
    pub const SMALL_LETTER_T: Correspondence<Linear> = Linear::new("ｔ", "t").corr();
    /// ｕ (全角小文字) -> u (半角小文字) の置換です。
    pub const SMALL_LETTER_U: Correspondence<Linear> = Linear::new("ｕ", "u").corr();
    /// ｖ (全角小文字) -> v (半角小文字) の置換です。
    pub const SMALL_LETTER_V: Correspondence<Linear> = Linear::new("ｖ", "v").corr();
    /// ｗ (全角小文字) -> w (半角小文字) の置換です。
    pub const SMALL_LETTER_W: Correspondence<Linear> = Linear::new("ｗ", "w").corr();
    /// ｘ (全角小文字) -> x (半角小文字) の置換です。
    pub const SMALL_LETTER_X: Correspondence<Linear> = Linear::new("ｘ", "x").corr();
    /// ｙ (全角小文字) -> y (半角小文字) の置換です。
    pub const SMALL_LETTER_Y: Correspondence<Linear> = Linear::new("ｙ", "y").corr();
    /// ｚ (全角小文字) -> z (半角小文字) の置換です。
    pub const SMALL_LETTER_Z: Correspondence<Linear> = Linear::new("ｚ", "z").corr();

    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

/// 全角アルファベットから半角アルファベットの置換テーブルの定数です。
pub const ALPHABETS: Correspondence<Alphabets> = Alphabets::new().corr();
