pub mod alphabets;
pub mod cjk_compatibilities;
pub mod digits;
pub mod kanas;
pub mod prelude;
pub mod punctuations;

use alphabets::ALPHABETS;
use cjk_compatibilities::CJK_COMPATIBILITIES;
use digits::DIGITS;
use kanas::KANAS;
use punctuations::PUNCTUATIONS;

use crate::chanoma::corr::{Corr, Correspondence, Item};

pub struct All;

impl Corr for All {
    fn items(&self) -> Vec<Item> {
        (DIGITS + &PUNCTUATIONS + &ALPHABETS + &KANAS + &CJK_COMPATIBILITIES).items()
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

pub const ALL: Correspondence<All> = All::new().corr();
