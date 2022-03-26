use crate::corr::{Corr, Correspondence, Item, Linear};

#[derive(Clone)]
pub struct Punctuations;

impl Corr for Punctuations {
    fn items(&self) -> Vec<Item> {
        (EXCLAMATION_MARK
            + &QUOTATION_MARK
            + &SHARP
            + &DOLLER_SIGN
            + &PERCENT_SIGN
            + &AMPERSAND
            + &APOSTROPHE
            + &LEFT_PARENTHESIS
            + &RIGHT_PARENTHESIS
            + &ASTERISK
            + &PLUS_SIGN
            + &COMMA
            + &HYPHEN_MINUS
            + &FULL_STOP
            + &SLASH
            + &COLON
            + &SEMI_COLON
            + &LESS_THAN_SIGN
            + &EQUAL_SIGN
            + &GREATER_THAN_SIGN
            + &QUESTION_MARK
            + &AT_SIGN
            + &LEFT_SQUARE_BRACKET
            + &BACKSLASH
            + &YEN_SIGN
            + &RIGHT_SQUARE_BRACKET
            + &CIRCUMFLEX_ACCENT
            + &LOW_LINE
            + &GRAVE_ACCENT
            + &LEFT_CURLY_BRACKET
            + &VERTICAL_BAR
            + &RIGHT_CURLY_BRACKET
            + &TILDE)
            .items()
    }
}

impl Punctuations {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

pub const EXCLAMATION_MARK: Correspondence<Linear> = Linear::new("！", "!").corr();
pub const QUOTATION_MARK: Correspondence<Linear> = Linear::new("”", "\"").corr();
pub const SHARP: Correspondence<Linear> = Linear::new("＃", "#").corr();
pub const DOLLER_SIGN: Correspondence<Linear> = Linear::new("＄", "$").corr();
pub const PERCENT_SIGN: Correspondence<Linear> = Linear::new("％", "%").corr();
pub const AMPERSAND: Correspondence<Linear> = Linear::new("＆", "&").corr();
pub const APOSTROPHE: Correspondence<Linear> = Linear::new("’", "'").corr();
pub const LEFT_PARENTHESIS: Correspondence<Linear> = Linear::new("（", "(").corr();
pub const RIGHT_PARENTHESIS: Correspondence<Linear> = Linear::new("）", ")").corr();
pub const ASTERISK: Correspondence<Linear> = Linear::new("＊", "*").corr();
pub const PLUS_SIGN: Correspondence<Linear> = Linear::new("＋", "+").corr();
pub const COMMA: Correspondence<Linear> = Linear::new("，", ",").corr();
pub const HYPHEN_MINUS: Correspondence<Linear> = Linear::new("－", "-").corr();
pub const FULL_STOP: Correspondence<Linear> = Linear::new("．", ".").corr();
pub const SLASH: Correspondence<Linear> = Linear::new("／", "/").corr();

pub const COLON: Correspondence<Linear> = Linear::new("：", ":").corr();
pub const SEMI_COLON: Correspondence<Linear> = Linear::new("；", ";").corr();
pub const LESS_THAN_SIGN: Correspondence<Linear> = Linear::new("＜", "<").corr();
pub const EQUAL_SIGN: Correspondence<Linear> = Linear::new("＝", "=").corr();
pub const GREATER_THAN_SIGN: Correspondence<Linear> = Linear::new("＞", ">").corr();
pub const QUESTION_MARK: Correspondence<Linear> = Linear::new("？", "?").corr();
pub const AT_SIGN: Correspondence<Linear> = Linear::new("＠", "@").corr();

pub const LEFT_SQUARE_BRACKET: Correspondence<Linear> = Linear::new("［", "[").corr();
pub const BACKSLASH: Correspondence<Linear> = Linear::new("＼", "\\").corr();
pub const YEN_SIGN: Correspondence<Linear> = Linear::new("￥", "¥").corr();
pub const RIGHT_SQUARE_BRACKET: Correspondence<Linear> = Linear::new("］", "]").corr();
pub const CIRCUMFLEX_ACCENT: Correspondence<Linear> = Linear::new("＾", "^").corr();
pub const LOW_LINE: Correspondence<Linear> = Linear::new("＿", "_").corr();
pub const GRAVE_ACCENT: Correspondence<Linear> = Linear::new("｀", "`").corr();

pub const LEFT_CURLY_BRACKET: Correspondence<Linear> = Linear::new("｛", "{").corr();
pub const VERTICAL_BAR: Correspondence<Linear> = Linear::new("｜", "|").corr();
pub const RIGHT_CURLY_BRACKET: Correspondence<Linear> = Linear::new("｝", "}").corr();
pub const TILDE: Correspondence<Linear> = Linear::new("〜", "~").corr();

pub const PUNCTUATIONS: Correspondence<Punctuations> = Punctuations::new().corr();
