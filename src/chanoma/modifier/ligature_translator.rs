use super::{ModifiedData, Modifier};
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::position::Position;
use crate::chanoma::error::ErrorKind;
use std::collections::HashMap;
use std::str::FromStr;

// 合字をくっつけて一文字にする
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LigatureTranslator {
    pub map: HashMap<String, String>,
}

impl Modifier for LigatureTranslator {
    fn modify(&self, input: &str) -> String {
        let mut text = input.to_string();
        for (from, to) in self.map.iter() {
            text = text.replace(from, to);
        }
        text
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        let positions: Vec<Position> = vec![];
        ModifiedData::new(
            ModifierKind::LigatureTranslator(self.clone()),
            self.modify(input),
            positions,
        )
    }
}

impl ModifierFromYamlValue for LigatureTranslator {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, ErrorKind> {
        let map = value
            .as_mapping()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let k = key.as_str().unwrap().to_string();
                let v = value.as_str().unwrap().to_string();
                (k, v)
            })
            .collect::<HashMap<String, String>>();
        Ok(Self::from_map(map))
    }
}

impl FromStr for LigatureTranslator {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ligature_translator(ハ゜,パ)
        let strs = s
            .split(',')
            .into_iter()
            .collect::<Vec<&str>>();
        if strs.len() != 2 {
            return Err(ErrorKind::ModifierKindParseError("Only two values.".to_string()));
        }
        let mut strs = strs.iter();
        let map = vec![(strs.next().unwrap().to_string(), strs.next().unwrap().to_string())]
            .into_iter()
            .collect::<HashMap<String, String>>();
        Ok(Self::from_map(map))
    }
}

impl From<LigatureTranslator> for ModifierKind {
    fn from(m: LigatureTranslator) -> ModifierKind {
        ModifierKind::LigatureTranslator(m)
    }
}

impl LigatureTranslator {
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self { map }
    }
}

/// ligature_translator!{"ハ゜" => 'パ'};
#[macro_export]
macro_rules! ligature_translator {
    ($($from:expr => $to:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($from.to_string(), $to.to_string());
        )*
        crate::chanoma::modifier::ligature_translator::LigatureTranslator::from_map(map)
    }};
}
