use super::{ModifiedRecord, Modifier};
use crate::error::Error;
use crate::modifier::ModifierFromYamlValue;
use crate::modifier_kind::ModifierKind;
use crate::position::Position;
use std::collections::HashSet;
use std::str::FromStr;

// 文字を削除する
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterEliminator {
    chars: HashSet<char>,
}

impl Modifier for CharacterEliminator {
    fn modify(&self, input: &str) -> String {
        let mut s = Vec::new();
        for c in input.chars() {
            if self.chars.get(&c).is_some() {
                // noop
            } else {
                s.push(c);
            }
        }
        s.into_iter().collect::<String>()
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedRecord {
        let positions: Vec<Position> = vec![];
        ModifiedRecord::new(
            ModifierKind::CharacterEliminator(self.clone()),
            self.modify(input),
            positions,
        )
    }
}

impl ModifierFromYamlValue for CharacterEliminator {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, Error> {
        let map = value
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().chars().next().unwrap())
            .collect::<Vec<char>>();
        Ok(Self::from_chars(map))
    }
}

impl FromStr for CharacterEliminator {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s
            .split(',')
            .into_iter()
            .filter_map(|st| st.trim().to_string().chars().next())
            .collect::<Vec<char>>();
        if chars.is_empty() {
            return Err(Error::ModifierKindParseError("require value.".to_string()));
        }
        Ok(Self::from_chars(chars))
    }
}

impl From<CharacterEliminator> for ModifierKind {
    fn from(m: CharacterEliminator) -> ModifierKind {
        ModifierKind::CharacterEliminator(m)
    }
}

impl CharacterEliminator {
    #[allow(dead_code)]
    pub fn from_char(c: char) -> Self {
        let mut chars = HashSet::new();
        chars.insert(c);
        Self { chars }
    }

    pub fn from_chars(chs: Vec<char>) -> Self {
        let mut chars = HashSet::new();
        for c in chs.into_iter() {
            chars.insert(c);
        }
        Self { chars }
    }
}

/// character_eliminator!['a', 'b'};
#[macro_export]
macro_rules! character_eliminator {
    ($($c:expr),*) => {{
        let mut ch = Vec::new();
        $(
            ch.push($c);
        )*
        crate::modifier::character_eliminator::CharacterEliminator::from_chars(ch)
    }};
}
