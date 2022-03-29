use super::{Corr, Correspondence, Item};

pub struct NCharactersToCharacter<'a> {
    froms: &'a [&'static str],
    to: &'static str,
}

impl<'a> Corr for NCharactersToCharacter<'a> {
    fn items(&self) -> Vec<Item> {
        self.froms
            .iter()
            .map(|from| Item::new(*from, self.to))
            .collect()
    }
}

impl<'a> NCharactersToCharacter<'a> {
    pub const fn new(froms: &'a [&'static str], to: &'static str) -> Self {
        Self { froms, to }
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

pub type Unification<'a> = NCharactersToCharacter<'a>;
