//! 全角記号から半角記号の置換テーブルを定義しているモジュールです。
use crate::corr::{Corr, Correspondence, Item, Linear};

/// 全角記号から半角記号の置換テーブルを保持する構造体です。
#[derive(Clone)]
pub struct Punctuations;

impl Corr for Punctuations {
    fn items(&self) -> Vec<Item> {
        (Self::EXCLAMATION_MARK
            + &Self::QUOTATION_MARK
            + &Self::SHARP
            + &Self::DOLLER_SIGN
            + &Self::PERCENT_SIGN
            + &Self::AMPERSAND
            + &Self::APOSTROPHE
            + &Self::LEFT_PARENTHESIS
            + &Self::RIGHT_PARENTHESIS
            + &Self::ASTERISK
            + &Self::PLUS_SIGN
            + &Self::COMMA
            + &Self::HYPHEN_MINUS
            + &Self::FULL_STOP
            + &Self::SLASH
            + &Self::COLON
            + &Self::SEMI_COLON
            + &Self::LESS_THAN_SIGN
            + &Self::EQUAL_SIGN
            + &Self::GREATER_THAN_SIGN
            + &Self::QUESTION_MARK
            + &Self::AT_SIGN
            + &Self::LEFT_SQUARE_BRACKET
            + &Self::BACKSLASH
            + &Self::YEN_SIGN
            + &Self::RIGHT_SQUARE_BRACKET
            + &Self::CIRCUMFLEX_ACCENT
            + &Self::LOW_LINE
            + &Self::GRAVE_ACCENT
            + &Self::LEFT_CURLY_BRACKET
            + &Self::VERTICAL_BAR
            + &Self::RIGHT_CURLY_BRACKET
            + &Self::TILDE)
            .items()
    }
}

impl Punctuations {
    /// エクスクラメーションマーク ！ (全角) -> ! (半角) の置換です。
    pub const EXCLAMATION_MARK: Correspondence<Linear> = Linear::new("！", "!").corr();
    /// ダブルクオーテーション ” (全角) -> " (半角) の置換です。
    pub const QUOTATION_MARK: Correspondence<Linear> = Linear::new("”", "\"").corr();
    /// シャープ ＃ (全角) -> # (半角) の置換です。
    pub const SHARP: Correspondence<Linear> = Linear::new("＃", "#").corr();
    /// ドル ＄ (全角) -> $ (半角) の置換です。
    pub const DOLLER_SIGN: Correspondence<Linear> = Linear::new("＄", "$").corr();
    /// パーセント ％ (全角) -> % (半角) の置換です。
    pub const PERCENT_SIGN: Correspondence<Linear> = Linear::new("％", "%").corr();
    /// アンパサンド ＆ (全角) -> & (半角) の置換です。
    pub const AMPERSAND: Correspondence<Linear> = Linear::new("＆", "&").corr();
    /// シングルクオーテーション ’ (全角) -> ' (半角) の置換です。
    pub const APOSTROPHE: Correspondence<Linear> = Linear::new("’", "'").corr();
    /// 丸括弧開き （ (全角) -> ( (半角) の置換です。
    pub const LEFT_PARENTHESIS: Correspondence<Linear> = Linear::new("（", "(").corr();
    /// 丸括弧閉じ ） (全角) -> ) (半角) の置換です。
    pub const RIGHT_PARENTHESIS: Correspondence<Linear> = Linear::new("）", ")").corr();
    /// アスタリスク ＊ (全角) -> * (半角) の置換です。
    pub const ASTERISK: Correspondence<Linear> = Linear::new("＊", "*").corr();
    /// プラス ＋ (全角) -> + (半角) の置換です。
    pub const PLUS_SIGN: Correspondence<Linear> = Linear::new("＋", "+").corr();
    /// カンマ ， (全角) -> , (半角) の置換です。
    pub const COMMA: Correspondence<Linear> = Linear::new("，", ",").corr();
    /// ハイフン － (全角) -> - (半角) の置換です。
    pub const HYPHEN_MINUS: Correspondence<Linear> = Linear::new("－", "-").corr();
    /// ピリオド ． (全角) -> . (半角) の置換です。
    pub const FULL_STOP: Correspondence<Linear> = Linear::new("．", ".").corr();
    /// スラッシュ ／ (全角) -> / (半角) の置換です。
    pub const SLASH: Correspondence<Linear> = Linear::new("／", "/").corr();

    /// コロン ： (全角) -> : (半角) の置換です。
    pub const COLON: Correspondence<Linear> = Linear::new("：", ":").corr();
    /// セミコロン ； (全角) -> ; (半角) の置換です。
    pub const SEMI_COLON: Correspondence<Linear> = Linear::new("；", ";").corr();
    /// 小なり ＜ (全角) -> < (半角) の置換です。
    pub const LESS_THAN_SIGN: Correspondence<Linear> = Linear::new("＜", "<").corr();
    /// イコール ＝ (全角) -> = (半角) の置換です。
    pub const EQUAL_SIGN: Correspondence<Linear> = Linear::new("＝", "=").corr();
    /// 大なり ＞ (全角) -> > (半角) の置換です。
    pub const GREATER_THAN_SIGN: Correspondence<Linear> = Linear::new("＞", ">").corr();
    /// クエスチョンマーク ？ (全角) -> ? (半角) の置換です。
    pub const QUESTION_MARK: Correspondence<Linear> = Linear::new("？", "?").corr();
    /// アットマーク ＠ (全角) -> @ (半角) の置換です。
    pub const AT_SIGN: Correspondence<Linear> = Linear::new("＠", "@").corr();

    /// 角括弧開き ［ (全角) -> [ (半角) の置換です。
    pub const LEFT_SQUARE_BRACKET: Correspondence<Linear> = Linear::new("［", "[").corr();
    /// バックスラッシュ ＼ (全角) -> \ (半角) の置換です。
    pub const BACKSLASH: Correspondence<Linear> = Linear::new("＼", "\\").corr();
    /// 円マーク ￥ (全角) -> ¥ (半角) の置換です。
    pub const YEN_SIGN: Correspondence<Linear> = Linear::new("￥", "¥").corr();
    /// 角括弧閉じ ］ (全角) -> ] (半角) の置換です。
    pub const RIGHT_SQUARE_BRACKET: Correspondence<Linear> = Linear::new("］", "]").corr();
    /// キャレット ＾ (全角) -> ^ (半角) の置換です。
    pub const CIRCUMFLEX_ACCENT: Correspondence<Linear> = Linear::new("＾", "^").corr();
    /// アンダースコア ＿ (全角) -> _ (半角) の置換です。
    pub const LOW_LINE: Correspondence<Linear> = Linear::new("＿", "_").corr();
    /// バッククオート ｀ (全角) -> ` (半角) の置換です。
    pub const GRAVE_ACCENT: Correspondence<Linear> = Linear::new("｀", "`").corr();

    /// 波括弧開き ｛ (全角) -> { (半角) の置換です。
    pub const LEFT_CURLY_BRACKET: Correspondence<Linear> = Linear::new("｛", "{").corr();
    /// バー ｜ (全角) -> | (半角) の置換です。
    pub const VERTICAL_BAR: Correspondence<Linear> = Linear::new("｜", "|").corr();
    /// 波括弧閉じ ｝ (全角) -> } (半角) の置換です。
    pub const RIGHT_CURLY_BRACKET: Correspondence<Linear> = Linear::new("｝", "}").corr();
    /// チルダ 〜 (全角) -> ~ (半角) の置換です。
    pub const TILDE: Correspondence<Linear> = Linear::new("〜", "~").corr();

    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

/// 全角記号から半角記号の置換テーブルの定数です。
pub const PUNCTUATIONS: Correspondence<Punctuations> = Punctuations::new().corr();
