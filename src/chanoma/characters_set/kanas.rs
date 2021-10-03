use crate::chanoma::corr::{Corr, Correspondence, Item, Linear};

#[derive(Clone)]
pub struct Kanas;

impl Corr for Kanas {
    fn items(&self) -> Vec<Item> {
        (KATAKANA_LETTER_SMALL_A
            + &KATAKANA_LETTER_A
            + &KATAKANA_LETTER_SMALL_I
            + &KATAKANA_LETTER_I
            + &KATAKANA_LETTER_SMALL_U
            + &KATAKANA_LETTER_U
            + &KATAKANA_LETTER_SMALL_E
            + &KATAKANA_LETTER_E
            + &KATAKANA_LETTER_SMALL_O
            + &KATAKANA_LETTER_O
            + &KATAKANA_LETTER_KA
            + &KATAKANA_LETTER_KI
            + &KATAKANA_LETTER_KU
            + &KATAKANA_LETTER_KE
            + &KATAKANA_LETTER_KO
            + &KATAKANA_LETTER_SA
            + &KATAKANA_LETTER_SI
            + &KATAKANA_LETTER_SU
            + &KATAKANA_LETTER_SE
            + &KATAKANA_LETTER_SO
            + &KATAKANA_LETTER_TA
            + &KATAKANA_LETTER_TI
            + &KATAKANA_LETTER_SMALL_TU
            + &KATAKANA_LETTER_TU
            + &KATAKANA_LETTER_TE
            + &KATAKANA_LETTER_TO
            + &KATAKANA_LETTER_NA
            + &KATAKANA_LETTER_NI
            + &KATAKANA_LETTER_NU
            + &KATAKANA_LETTER_NE
            + &KATAKANA_LETTER_NO
            + &KATAKANA_LETTER_HA
            + &KATAKANA_LETTER_HI
            + &KATAKANA_LETTER_HU
            + &KATAKANA_LETTER_HE
            + &KATAKANA_LETTER_HO
            + &KATAKANA_LETTER_MA
            + &KATAKANA_LETTER_MI
            + &KATAKANA_LETTER_MU
            + &KATAKANA_LETTER_ME
            + &KATAKANA_LETTER_MO
            + &KATAKANA_LETTER_SMALL_YA
            + &KATAKANA_LETTER_YA
            + &KATAKANA_LETTER_SMALL_YU
            + &KATAKANA_LETTER_YU
            + &KATAKANA_LETTER_SMALL_YO
            + &KATAKANA_LETTER_YO
            + &KATAKANA_LETTER_RA
            + &KATAKANA_LETTER_RI
            + &KATAKANA_LETTER_RU
            + &KATAKANA_LETTER_RE
            + &KATAKANA_LETTER_RO
            + &KATAKANA_LETTER_WA
            + &KATAKANA_LETTER_WO
            + &KATAKANA_LETTER_N
            + &IDEOGRAPHIC_COMMA
            + &IDEOGRAPHIC_FULL_STOP
            + &KATAKANA_MIDDLE_DOT
            + &LEFT_CORNER_BRACKET
            + &RIGHT_CORNER_BRACKET
            + &IDEOGRAPHIC_SPACE)
            .items()
    }
}

impl Kanas {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

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

pub const KANAS: Correspondence<Kanas> = Kanas::new().corr();
