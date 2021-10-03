use super::{Corr, Correspondence, Item};

#[derive(Clone)]
pub struct CharacterToCharacter {
    from: &'static str,
    to: &'static str,
}

impl Corr for CharacterToCharacter {
    fn items(&self) -> Vec<Item> {
        vec![Item::new(self.from, self.to)]
    }
}

impl CharacterToCharacter {
    pub const fn new(from: &'static str, to: &'static str) -> Self {
        Self { from, to }
    }

    pub const fn corr(self) -> Correspondence<Self> {
        Correspondence::new(self)
    }
}

pub type Linear = CharacterToCharacter;
