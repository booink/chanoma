use super::{ModifiedData, Modifier};
use crate::chanoma::error::Error;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::position::Position;
use std::str::FromStr;

// 『「半角英数字」と「半角英数字」の間の半角スペース』以外の半角スペースを削除する
// 結果的にTrimとConsecutiveCharacterReducer{c: ' '}と同様の処理もされる
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DottedSpaceEliminator;

impl Modifier for DottedSpaceEliminator {
    fn modify(&self, input: &str) -> String {
        let mut text = String::new();
        let mut prev = ' ';
        let mut space = false;
        let mut buffer = [0u8; 4]; // char型から&strに変換する際に.to_string()でString型を経由するとメモリの動的確保によるオーバーヘッドがかかるので、直接&strに変換するためのバッファ
        for c in input.chars() {
            if c == ' ' {
                // 前の文字が半角英数字
                if prev.is_ascii_alphanumeric() {
                    space = true;
                }
            } else {
                if space {
                    if c.is_ascii_alphanumeric() {
                        text.push(' ');
                    }
                    space = false;
                }
                text.push_str(c.encode_utf8(&mut buffer));
            }
            prev = c;
        }
        text
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        let positions: Vec<Position> = vec![];
        ModifiedData::new(
            ModifierKind::DottedSpaceEliminator(self.clone()),
            self.modify(input),
            positions,
        )
    }
}

impl ModifierFromYamlValue for DottedSpaceEliminator {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, Error> {
        if value.is_null() {
            Ok(Self::new())
        } else {
            Err(Error::ModifierKindParseError(
                "Cannot specify a value.".to_string(),
            ))
        }
    }
}

impl FromStr for DottedSpaceEliminator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s.is_empty() {
            Ok(Self::new())
        } else {
            Err(Error::ModifierKindParseError("Cannot specify a value.".to_string()).into())
        }
    }
}

impl From<DottedSpaceEliminator> for ModifierKind {
    fn from(m: DottedSpaceEliminator) -> ModifierKind {
        ModifierKind::DottedSpaceEliminator(m)
    }
}

impl Default for DottedSpaceEliminator {
    fn default() -> Self {
        Self::new()
    }
}

impl DottedSpaceEliminator {
    pub fn new() -> Self {
        Self {}
    }
}
