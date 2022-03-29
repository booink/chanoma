//! 半角カタカナから全角カタカナの置換テーブルを定義しているモジュールです。
use crate::corr::{Corr, Correspondence, Item, Linear};

/// 半角カタカナから全角カタカナの置換テーブルを保持する構造体です。
#[derive(Clone)]
pub struct Kanas;

impl Corr for Kanas {
    fn items(&self) -> Vec<Item> {
        (Self::KATAKANA_LETTER_SMALL_A
            + &Self::KATAKANA_LETTER_A
            + &Self::KATAKANA_LETTER_SMALL_I
            + &Self::KATAKANA_LETTER_I
            + &Self::KATAKANA_LETTER_SMALL_U
            + &Self::KATAKANA_LETTER_U
            + &Self::KATAKANA_LETTER_SMALL_E
            + &Self::KATAKANA_LETTER_E
            + &Self::KATAKANA_LETTER_SMALL_O
            + &Self::KATAKANA_LETTER_O
            + &Self::KATAKANA_LETTER_KA
            + &Self::KATAKANA_LETTER_KI
            + &Self::KATAKANA_LETTER_KU
            + &Self::KATAKANA_LETTER_KE
            + &Self::KATAKANA_LETTER_KO
            + &Self::KATAKANA_LETTER_SA
            + &Self::KATAKANA_LETTER_SI
            + &Self::KATAKANA_LETTER_SU
            + &Self::KATAKANA_LETTER_SE
            + &Self::KATAKANA_LETTER_SO
            + &Self::KATAKANA_LETTER_TA
            + &Self::KATAKANA_LETTER_TI
            + &Self::KATAKANA_LETTER_SMALL_TU
            + &Self::KATAKANA_LETTER_TU
            + &Self::KATAKANA_LETTER_TE
            + &Self::KATAKANA_LETTER_TO
            + &Self::KATAKANA_LETTER_NA
            + &Self::KATAKANA_LETTER_NI
            + &Self::KATAKANA_LETTER_NU
            + &Self::KATAKANA_LETTER_NE
            + &Self::KATAKANA_LETTER_NO
            + &Self::KATAKANA_LETTER_HA
            + &Self::KATAKANA_LETTER_HI
            + &Self::KATAKANA_LETTER_HU
            + &Self::KATAKANA_LETTER_HE
            + &Self::KATAKANA_LETTER_HO
            + &Self::KATAKANA_LETTER_MA
            + &Self::KATAKANA_LETTER_MI
            + &Self::KATAKANA_LETTER_MU
            + &Self::KATAKANA_LETTER_ME
            + &Self::KATAKANA_LETTER_MO
            + &Self::KATAKANA_LETTER_SMALL_YA
            + &Self::KATAKANA_LETTER_YA
            + &Self::KATAKANA_LETTER_SMALL_YU
            + &Self::KATAKANA_LETTER_YU
            + &Self::KATAKANA_LETTER_SMALL_YO
            + &Self::KATAKANA_LETTER_YO
            + &Self::KATAKANA_LETTER_RA
            + &Self::KATAKANA_LETTER_RI
            + &Self::KATAKANA_LETTER_RU
            + &Self::KATAKANA_LETTER_RE
            + &Self::KATAKANA_LETTER_RO
            + &Self::KATAKANA_LETTER_WA
            + &Self::KATAKANA_LETTER_WO
            + &Self::KATAKANA_LETTER_N
            + &Self::IDEOGRAPHIC_COMMA
            + &Self::IDEOGRAPHIC_FULL_STOP
            + &Self::KATAKANA_MIDDLE_DOT
            + &Self::LEFT_CORNER_BRACKET
            + &Self::RIGHT_CORNER_BRACKET
            + &Self::IDEOGRAPHIC_SPACE)
            .items()
    }
}

impl Kanas {
    /// ｧ (半角) -> ァ (全角) の置換です。
    pub const KATAKANA_LETTER_SMALL_A: Correspondence<Linear> = Linear::new("ｧ", "ァ").corr();
    pub const KATAKANA_LETTER_A: Correspondence<Linear> = Linear::new("ｱ", "ア").corr();
    pub const KATAKANA_LETTER_SMALL_I: Correspondence<Linear> = Linear::new("ｨ", "ィ").corr();
    pub const KATAKANA_LETTER_I: Correspondence<Linear> = Linear::new("ｲ", "イ").corr();
    pub const KATAKANA_LETTER_SMALL_U: Correspondence<Linear> = Linear::new("ｩ", "ゥ").corr();
    pub const KATAKANA_LETTER_U: Correspondence<Linear> = Linear::new("ｳ", "ウ").corr();
    pub const KATAKANA_LETTER_SMALL_E: Correspondence<Linear> = Linear::new("ｪ", "ェ").corr();
    pub const KATAKANA_LETTER_E: Correspondence<Linear> = Linear::new("ｴ", "エ").corr();
    pub const KATAKANA_LETTER_SMALL_O: Correspondence<Linear> = Linear::new("ｫ", "ォ").corr();
    pub const KATAKANA_LETTER_O: Correspondence<Linear> = Linear::new("ｵ", "オ").corr();

    pub const KATAKANA_LETTER_KA: Correspondence<Linear> = Linear::new("ｶ", "カ").corr();
    pub const KATAKANA_LETTER_KI: Correspondence<Linear> = Linear::new("ｷ", "キ").corr();
    pub const KATAKANA_LETTER_KU: Correspondence<Linear> = Linear::new("ｸ", "ク").corr();
    pub const KATAKANA_LETTER_KE: Correspondence<Linear> = Linear::new("ｹ", "ケ").corr();
    pub const KATAKANA_LETTER_KO: Correspondence<Linear> = Linear::new("ｺ", "コ").corr();

    pub const KATAKANA_LETTER_SA: Correspondence<Linear> = Linear::new("ｻ", "サ").corr();
    pub const KATAKANA_LETTER_SI: Correspondence<Linear> = Linear::new("ｼ", "シ").corr();
    pub const KATAKANA_LETTER_SU: Correspondence<Linear> = Linear::new("ｽ", "ス").corr();
    pub const KATAKANA_LETTER_SE: Correspondence<Linear> = Linear::new("ｾ", "セ").corr();
    pub const KATAKANA_LETTER_SO: Correspondence<Linear> = Linear::new("ｿ", "ソ").corr();

    pub const KATAKANA_LETTER_TA: Correspondence<Linear> = Linear::new("ﾀ", "タ").corr();
    pub const KATAKANA_LETTER_TI: Correspondence<Linear> = Linear::new("ﾁ", "チ").corr();
    pub const KATAKANA_LETTER_SMALL_TU: Correspondence<Linear> = Linear::new("ｯ", "ッ").corr();
    pub const KATAKANA_LETTER_TU: Correspondence<Linear> = Linear::new("ﾂ", "ツ").corr();
    pub const KATAKANA_LETTER_TE: Correspondence<Linear> = Linear::new("ﾃ", "テ").corr();
    pub const KATAKANA_LETTER_TO: Correspondence<Linear> = Linear::new("ﾄ", "ト").corr();

    pub const KATAKANA_LETTER_NA: Correspondence<Linear> = Linear::new("ﾅ", "ナ").corr();
    pub const KATAKANA_LETTER_NI: Correspondence<Linear> = Linear::new("ﾆ", "ニ").corr();
    pub const KATAKANA_LETTER_NU: Correspondence<Linear> = Linear::new("ﾇ", "ヌ").corr();
    pub const KATAKANA_LETTER_NE: Correspondence<Linear> = Linear::new("ﾈ", "ネ").corr();
    pub const KATAKANA_LETTER_NO: Correspondence<Linear> = Linear::new("ﾉ", "ノ").corr();

    pub const KATAKANA_LETTER_HA: Correspondence<Linear> = Linear::new("ﾊ", "ハ").corr();
    pub const KATAKANA_LETTER_HI: Correspondence<Linear> = Linear::new("ﾋ", "ヒ").corr();
    pub const KATAKANA_LETTER_HU: Correspondence<Linear> = Linear::new("ﾌ", "フ").corr();
    pub const KATAKANA_LETTER_HE: Correspondence<Linear> = Linear::new("ﾍ", "ヘ").corr();
    pub const KATAKANA_LETTER_HO: Correspondence<Linear> = Linear::new("ﾎ", "ホ").corr();

    pub const KATAKANA_LETTER_MA: Correspondence<Linear> = Linear::new("ﾏ", "マ").corr();
    pub const KATAKANA_LETTER_MI: Correspondence<Linear> = Linear::new("ﾐ", "ミ").corr();
    pub const KATAKANA_LETTER_MU: Correspondence<Linear> = Linear::new("ﾑ", "ム").corr();
    pub const KATAKANA_LETTER_ME: Correspondence<Linear> = Linear::new("ﾒ", "メ").corr();
    pub const KATAKANA_LETTER_MO: Correspondence<Linear> = Linear::new("ﾓ", "モ").corr();

    pub const KATAKANA_LETTER_SMALL_YA: Correspondence<Linear> = Linear::new("ｬ", "ャ").corr();
    pub const KATAKANA_LETTER_YA: Correspondence<Linear> = Linear::new("ﾔ", "ヤ").corr();
    pub const KATAKANA_LETTER_SMALL_YU: Correspondence<Linear> = Linear::new("ｭ", "ュ").corr();
    pub const KATAKANA_LETTER_YU: Correspondence<Linear> = Linear::new("ﾕ", "ユ").corr();
    pub const KATAKANA_LETTER_SMALL_YO: Correspondence<Linear> = Linear::new("ｮ", "ョ").corr();
    pub const KATAKANA_LETTER_YO: Correspondence<Linear> = Linear::new("ﾖ", "ヨ").corr();

    pub const KATAKANA_LETTER_RA: Correspondence<Linear> = Linear::new("ﾗ", "ラ").corr();
    pub const KATAKANA_LETTER_RI: Correspondence<Linear> = Linear::new("ﾘ", "リ").corr();
    pub const KATAKANA_LETTER_RU: Correspondence<Linear> = Linear::new("ﾙ", "ル").corr();
    pub const KATAKANA_LETTER_RE: Correspondence<Linear> = Linear::new("ﾚ", "レ").corr();
    pub const KATAKANA_LETTER_RO: Correspondence<Linear> = Linear::new("ﾛ", "ロ").corr();

    pub const KATAKANA_LETTER_WA: Correspondence<Linear> = Linear::new("ﾜ", "ワ").corr();
    pub const KATAKANA_LETTER_WO: Correspondence<Linear> = Linear::new("ｦ", "ヲ").corr();
    pub const KATAKANA_LETTER_N: Correspondence<Linear> = Linear::new("ﾝ", "ン").corr();

    pub const IDEOGRAPHIC_COMMA: Correspondence<Linear> = Linear::new("､", "、").corr();
    pub const IDEOGRAPHIC_FULL_STOP: Correspondence<Linear> = Linear::new("｡", "。").corr();
    pub const KATAKANA_MIDDLE_DOT: Correspondence<Linear> = Linear::new("･", "・").corr();
    pub const LEFT_CORNER_BRACKET: Correspondence<Linear> = Linear::new("｢", "「").corr();
    pub const RIGHT_CORNER_BRACKET: Correspondence<Linear> = Linear::new("｣", "」").corr();

    pub const IDEOGRAPHIC_SPACE: Correspondence<Linear> = Linear::new("　", " ").corr();

    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

/// 半角カタカナから全角カタカナの置換テーブルの定数です。
pub const KANAS: Correspondence<Kanas> = Kanas::new().corr();
