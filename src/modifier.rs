pub mod character_converter;
pub mod character_eliminator;
pub mod consecutive_character_reducer;
pub mod dotted_space_eliminator;
pub mod ligature_translator;
pub mod neologdn;
pub mod trim;

use crate::error::Error;
use crate::modifier_kind::ModifierKind;
use crate::position::Position;
pub use character_converter::CharacterConverter;
pub use character_eliminator::CharacterEliminator;
pub use consecutive_character_reducer::ConsecutiveCharacterReducer;
pub use dotted_space_eliminator::DottedSpaceEliminator;
pub use ligature_translator::LigatureTranslator;
pub use neologdn::Neologdn;
pub use trim::Trim;

pub trait Modifier {
    fn modify(&self, text: &str) -> String;
    fn modify_with_positions(&self, text: &str) -> ModifiedRecord;
}

#[derive(Clone, Debug)]
pub struct ModifiedRecord {
    pub kind: ModifierKind,
    pub text: String,
    pub positions: Vec<Position>,
}

impl ModifiedRecord {
    pub fn new(kind: ModifierKind, text: String, positions: Vec<Position>) -> Self {
        Self {
            kind,
            text,
            positions,
        }
    }
}

pub trait ModifierFromYamlValue {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, Error>
    where
        Self: Sized;
}

/// 実行された正規化処理の途中経過を保存する構造体です。
#[derive(Debug)]
pub struct ModifiedRecords {
    records: Vec<ModifiedRecord>,
}

impl ModifiedRecords {
    // [ModifiedRecords] 構造体を初期化します。
    //
    // ```
    // use chanoma::modifier::ModifiedRecords;
    //
    // let modified = ModifiedRecords::new(vec![]);
    // ```
    pub fn new(records: Vec<ModifiedRecord>) -> Self {
        Self { records }
    }

    // [ModifiedResult] 構造体から最終結果の文字列を取得します。
    //
    // ```
    // use chanoma::modifier::ModifiedRecords;
    //
    // let modified = ModifiedRecords::new(vec![]);
    // assert_eq!(modified.text(), "");
    // ```
    pub fn text(&self) -> &str {
        &self.records.last().unwrap().text
    }

    // [ModifiedRecords] 構造体からすべての途中経過を取得します。
    //
    // ```
    // use chanoma::modifier::ModifiedRecords;
    //
    // let modified = ModifiedRecords::new(vec![]);
    // assert_eq!(modified.modified_records(), vec![]);
    // ```
    pub fn modified_records(&self) -> &Vec<ModifiedRecord> {
        &self.records
    }
}
