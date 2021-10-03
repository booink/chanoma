use super::{ModifiedData, Modifier};
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::error::ErrorKind;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::position::Position;
use std::str::FromStr;

// 先頭と末尾の半角スペースを削除する
// neologdnのときにはSpaceEliminatorがTrimもするので、不要
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trim;

impl Modifier for Trim {
    fn modify(&self, input: &str) -> String {
        input.trim_matches(' ').to_string()
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        let mut positions = Vec::new();
        let t = input.trim_start_matches(' ');
        if t != input {
            let len = input.len() - t.len();
            positions.push(Position {
                from: " ".repeat(len),
                to: "".to_string(),
                index: 0,
                length: len,
                utf8_index: 0,
                utf8_length: len,
            });
        }
        let text = t.trim_end_matches(' ');
        if text != t {
            let len = t.len() - text.len();
            positions.push(Position {
                from: " ".repeat(len),
                to: "".to_string(),
                index: input.len() - len,
                length: len,
                utf8_index: input.chars().count() - len,
                utf8_length: len,
            });
        }

        ModifiedData::new(ModifierKind::Trim(self.clone()), text.to_string(), positions)
    }
}

impl ModifierFromYamlValue for Trim {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, ErrorKind> {
        if value.is_null() {
            Ok(Self::new())
        } else {
            Err(ErrorKind::ModifierKindParseError("Cannot specify a value.".to_string()))
        }
    }
}

impl FromStr for Trim {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self::new())
        } else {
            Err(ErrorKind::ModifierKindParseError("Cannot specify a value.".to_string()))
        }
    }
}

impl From<Trim> for ModifierKind {
    fn from(m: Trim) -> ModifierKind {
        ModifierKind::Trim(m)
    }
}

impl Default for Trim {
    fn default() -> Self {
        Self::new()
    }
}

impl Trim {
    pub fn new() -> Self {
        Self {}
    }
}
