//! 全角数字から半角数字の置換テーブルを定義しているモジュールです。
use crate::corr::{Corr, Correspondence, Item, Linear};

/// 全角数字から半角数字の置換テーブルを保持する構造体です。
#[derive(Clone)]
pub struct Digits;

impl Corr for Digits {
    fn items(&self) -> Vec<Item> {
        (Self::DIGIT_ZERO
            + &Self::DIGIT_ONE
            + &Self::DIGIT_TWO
            + &Self::DIGIT_THREE
            + &Self::DIGIT_FOUR
            + &Self::DIGIT_FIVE
            + &Self::DIGIT_SIX
            + &Self::DIGIT_SEVEN
            + &Self::DIGIT_EIGHT
            + &Self::DIGIT_NINE)
            .items()
    }
}

impl Digits {
    /// ０ (全角) -> 0 (半角) の置換です。
    pub const DIGIT_ZERO: Correspondence<Linear> = Linear::new("０", "0").corr();
    /// １ (全角) -> 1 (半角) の置換です。
    pub const DIGIT_ONE: Correspondence<Linear> = Linear::new("１", "1").corr();
    /// ２ (全角) -> 2 (半角) の置換です。
    pub const DIGIT_TWO: Correspondence<Linear> = Linear::new("２", "2").corr();
    /// ３ (全角) -> 3 (半角) の置換です。
    pub const DIGIT_THREE: Correspondence<Linear> = Linear::new("３", "3").corr();
    /// ４ (全角) -> 4 (半角) の置換です。
    pub const DIGIT_FOUR: Correspondence<Linear> = Linear::new("４", "4").corr();
    /// ５ (全角) -> 5 (半角) の置換です。
    pub const DIGIT_FIVE: Correspondence<Linear> = Linear::new("５", "5").corr();
    /// ６ (全角) -> 6 (半角) の置換です。
    pub const DIGIT_SIX: Correspondence<Linear> = Linear::new("６", "6").corr();
    /// ７ (全角) -> 7 (半角) の置換です。
    pub const DIGIT_SEVEN: Correspondence<Linear> = Linear::new("７", "7").corr();
    /// ８ (全角) -> 8 (半角) の置換です。
    pub const DIGIT_EIGHT: Correspondence<Linear> = Linear::new("８", "8").corr();
    /// ９ (全角) -> 9 (半角) の置換です。
    pub const DIGIT_NINE: Correspondence<Linear> = Linear::new("９", "9").corr();

    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

/// 全角数字から半角数字の置換テーブルの定数です。
pub const DIGITS: Correspondence<Digits> = Digits::new().corr();
