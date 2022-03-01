pub mod characters_set;
mod configuration;
pub mod corr;
pub mod error;
pub mod file;
pub mod file_extension;
mod modifier;
mod modifier_kind;
pub mod position;
pub mod table;

use configuration::Configuration;
use modifier::{CharacterConverter, ModifiedData, Modifier, Neologdn};
use modifier_kind::ModifierKind;
use table::{Table, TableBuilder};

pub struct Chanoma {
    modifiers: Vec<ModifierKind>,
}

impl Default for Chanoma {
    fn default() -> Self {
        Self::new()
    }
}

impl Chanoma {
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    pub fn load_rc(debug: bool) -> anyhow::Result<Self> {
        Ok(Self {
            modifiers: Configuration::new(debug).load()?,
        })
    }

    pub fn from_table(table: Table) -> Self {
        Self {
            modifiers: vec![CharacterConverter::from_tables(vec![table]).into()],
        }
    }

    pub fn add_table(&mut self, table: Table) -> &mut Self {
        self.modifiers
            .push(CharacterConverter::from_tables(vec![table]).into());
        self
    }

    pub fn from_modifier_kind(modifier_kind: ModifierKind) -> Self {
        Self {
            modifiers: vec![modifier_kind],
        }
    }

    fn all_characters_set() -> ModifierKind {
        CharacterConverter::from_tables(vec![TableBuilder::new().preset().build().clone()]).into()
    }

    pub fn preset() -> Self {
        Self {
            modifiers: vec![Self::all_characters_set()],
        }
    }

    pub fn use_preset(&mut self) -> &mut Self {
        self.modifiers.push(Self::all_characters_set());
        self
    }

    pub fn neologdn() -> Self {
        Self {
            modifiers: vec![Neologdn::new().into()],
        }
    }

    pub fn use_neologdn(&mut self) -> &mut Self {
        self.modifiers.push(Neologdn::new().into());
        self
    }

    pub fn add_modifier_kind(&mut self, modifier_kind: ModifierKind) -> &mut Self {
        self.modifiers.push(modifier_kind);
        self
    }

    pub fn normalize(&self, text: &str) -> String {
        self.modifiers
            .iter()
            .fold(text.to_string(), |acc, x| x.modify(&acc))
    }

    pub fn normalize_with_positions(&self, text: &str) -> ModifiedResult {
        let mut v = Vec::new();
        let mut text = text.to_string();
        for m in self.modifiers.iter() {
            let result = m.modify_with_positions(&text);
            v.push(result.clone());
            text = result.text;
        }
        ModifiedResult::new(v)
    }
}

pub struct ModifiedResult {
    data: Vec<ModifiedData>,
}

impl ModifiedResult {
    pub fn new(data: Vec<ModifiedData>) -> Self {
        Self { data }
    }

    pub fn text(&self) -> &str {
        &self.data.last().unwrap().text
    }

    pub fn modified_records(&self) -> &Vec<ModifiedData> {
        &self.data
    }
}
