use super::{ModifiedData, Modifier};
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::position::Position;
use crate::chanoma::error::ErrorKind;
use std::str::FromStr;

// 連続する同じ文字を一つにする
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsecutiveCharacterReducer {
    c: char,
}

impl Modifier for ConsecutiveCharacterReducer {
    fn modify(&self, input: &str) -> String {
        let mut consecutive = false; // 文字が連続しているか
        let mut text = String::new(); // 返却する文字列用のバッファ
        let mut buffer = [0u8; 4]; // char型から&strに変換する際に.to_string()でString型を経由するとメモリの動的確保によるオーバーヘッドがかかるので、直接&strに変換するためのバッファ
        for ch in input.chars() {
            if self.c == ch {
                if consecutive {
                    continue;
                } else {
                    consecutive = true;
                }
            } else if consecutive {
                consecutive = false;
            }
            text.push_str(ch.encode_utf8(&mut buffer));
        }

        text
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        let c_len = self.c.len_utf8();
        let mut counter = 0; // 何文字連続しているか
        let mut text = String::new(); // 返却する文字列用のバッファ
        let mut positions = Vec::new(); // 変更された位置情報
        let mut buffer = [0u8; 4]; // char型から&strに変換する際に.to_string()でString型を経由するとメモリの動的確保によるオーバーヘッドがかかるので、直接&strに変換するためのバッファ
        for (i, ch) in input.chars().enumerate() {
            if self.c == ch {
                if counter > 0 {
                    counter += 1;
                    continue;
                } else {
                    counter = 1;
                }
            } else if counter > 0 {
                positions.push(Position {
                    from: self.c.to_string().repeat(counter),
                    to: self.c.to_string(),
                    index: text.len(),
                    length: c_len * counter,
                    utf8_index: i,
                    utf8_length: counter,
                });
                counter = 0;
            }
            text.push_str(ch.encode_utf8(&mut buffer));
        }

        if counter > 0 {
            positions.push(Position {
                from: self.c.to_string().repeat(counter),
                to: self.c.to_string(),
                index: text.len(),
                length: c_len * counter,
                utf8_index: input.len() - counter,
                utf8_length: counter,
            });
        }
        ModifiedData::new(
            ModifierKind::ConsecutiveCharacterReducer(self.clone()),
            text,
            positions,
        )
    }
}

impl ModifierFromYamlValue for ConsecutiveCharacterReducer {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, ErrorKind> {
        Self::from_str(value.as_str().unwrap())
    }
}

impl FromStr for ConsecutiveCharacterReducer {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        Ok(Self::new(c))
    }
}

impl From<ConsecutiveCharacterReducer> for ModifierKind {
    fn from(m: ConsecutiveCharacterReducer) -> ModifierKind {
        ModifierKind::ConsecutiveCharacterReducer(m)
    }
}

impl ConsecutiveCharacterReducer {
    pub fn new(c: char) -> Self {
        Self { c }
    }
}

/// consecutive_character_reducer!('ー');
#[macro_export]
macro_rules! consecutive_character_reducer {
    ($c:expr) => {{
        crate::chanoma::modifier::consecutive_character_reducer::ConsecutiveCharacterReducer::new($c)
    }};
}
