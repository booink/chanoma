pub mod character_converter;
pub mod character_eliminator;
pub mod consecutive_character_reducer;
pub mod dotted_space_eliminator;
pub mod ligature_translator;
pub mod neologdn;
pub mod trim;

use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::position::Position;
use crate::chanoma::error::ErrorKind;
pub use character_converter::CharacterConverter;
pub use character_eliminator::CharacterEliminator;
pub use consecutive_character_reducer::ConsecutiveCharacterReducer;
pub use dotted_space_eliminator::DottedSpaceEliminator;
pub use ligature_translator::LigatureTranslator;
pub use neologdn::Neologdn;
pub use trim::Trim;

pub trait Modifier {
    fn modify(&self, text: &str) -> String;
    fn modify_with_positions(&self, text: &str) -> ModifiedData;
}

#[derive(Clone, Debug)]
pub struct ModifiedData {
    pub kind: ModifierKind,
    pub text: String,
    pub positions: Vec<Position>,
}

impl ModifiedData {
    pub fn new(kind: ModifierKind, text: String, positions: Vec<Position>) -> Self {
        Self {
            kind,
            text,
            positions,
        }
    }
}

pub trait ModifierFromYamlValue {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, ErrorKind> where Self: Sized;
}
