//! 定義済みの置換テーブル用のモジュールです。
pub mod prelude;

pub mod alphabets;
pub mod cjk_compatibilities;
pub mod digits;
pub mod kanas;
pub mod punctuations;

use alphabets::ALPHABETS;
use digits::DIGITS;
use kanas::KANAS;
use punctuations::PUNCTUATIONS;

#[allow(unused_imports)]
use cjk_compatibilities::CJK_COMPATIBILITIES;

use crate::corr::{Corr, Correspondence, Item};

/// すべての定義済みの置換テーブルを保持する構造体です。
pub struct All;

impl Corr for All {
    fn items(&self) -> Vec<Item> {
        // TODO: CJK_COMPATIBILITIES は「一文字→複数文字列」なので、charactor_converterにpresetできない。別の Modifier を作って対応する
        (DIGITS + &PUNCTUATIONS + &ALPHABETS + &KANAS).items()
    }
}

impl All {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

/// すべての定義済みの置換テーブルの定数です。
pub const ALL: Correspondence<All> = All::new().corr();
