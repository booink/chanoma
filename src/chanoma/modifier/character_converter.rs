use super::{ModifiedData, Modifier};
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::position::Position;
use crate::chanoma::table::Table;
use crate::chanoma::error::ErrorKind;
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

// utf8 validな1つのcharを格納する構造体
// utf8 validなのでutf8 lengthは常に1
// lengthは一文字につき一度計算すればあとは全部同じなので、indexだけ追加していく
struct CharPos {
    length: usize,
    indices: Vec<usize>,
    utf8_indices: Vec<usize>,
}

impl CharPos {
    const UTF8_LEN: usize = 1;
    pub fn new(length: usize) -> Self {
        Self {
            length,
            indices: Vec::new(),
            utf8_indices: Vec::new(),
        }
    }

    pub fn push_indexes(&mut self, index: usize, utf8_index: usize) {
        self.indices.push(index);
        self.utf8_indices.push(utf8_index);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterConverter {
    pub items_map: HashMap<char, char>,
}

impl Modifier for CharacterConverter {
    fn modify(&self, input: &str) -> String {
        let mut refs = Vec::new();
        let mut map = HashMap::new();
        for (i, c) in input.chars().enumerate() {
            refs.push(c);
            map.entry(c).or_insert_with(Vec::new).push(i);
        }

        for (c, indices) in map.iter() {
            if let Some(to) = self.items_map.get(c) {
                // 変換テーブルに該当の文字があったら、HashSetの該当の文字を変換後の文字に入れ替える
                // mem::replace で参照先の中身を入れ替えることでrefsの参照先を変えない
                for i in indices.iter() {
                    let _ = mem::replace(&mut refs[*i], *to);
                }
            }
        }
        refs.into_iter().collect::<String>()
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        // 文字をhashsetに登録して、そのインデックスか参照を配列に登録する
        // 正規化対象の文字をHashSetで入れ替える
        // 配列から元の文字列に直していく
        let mut refs = Vec::new();
        let mut map = HashMap::new();
        for (index, (i, c)) in input.char_indices().enumerate() {
            refs.push(c);
            map.entry(c)
                .or_insert_with(|| CharPos::new(c.len_utf8()))
                .push_indexes(index, i);
        }

        let mut positions = Vec::new();
        for (c, char_pos) in map.iter() {
            if let Some(to) = self.items_map.get(c) {
                // 変換テーブルに該当の文字があったら、HashSetの該当の文字を変換後の文字に入れ替える
                // mem::replace で参照先の中身を入れ替えることでrefsの参照先を変えない
                for (i, c_index) in char_pos.indices.iter().enumerate() {
                    let _ = mem::replace(&mut refs[*c_index], *to);
                    positions.push(Position::new(
                        c.to_string(),
                        to.to_string(),
                        *c_index,
                        char_pos.utf8_indices[i],
                        char_pos.length,
                        CharPos::UTF8_LEN,
                    ));
                }
            }
        }

        ModifiedData::new(
            ModifierKind::CharacterConverter(self.clone()),
            refs.into_iter().collect::<String>(),
            positions,
        )
    }
}

impl ModifierFromYamlValue for CharacterConverter {
    fn from_yaml_value(value: &serde_yaml::Value) -> Result<Self, ErrorKind> {
        let map = value
            .as_mapping()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let k = key.as_str().unwrap().chars().next().unwrap();
                let v = value.as_str().unwrap().chars().next().unwrap();
                (k, v)
            })
            .collect::<HashMap<char, char>>();
        Ok(Self::from_map(map))
    }
}

impl FromStr for CharacterConverter {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s
            .split(',')
            .into_iter()
            .filter_map(|st| {
                let cs = st.trim()
                    .split(':')
                    .into_iter()
                    .filter_map(|c| c.trim().to_string().chars().next())
                    .collect::<Vec<char>>();
                if cs.len() != 2 {
                    None
                } else {
                    let mut cs = cs.into_iter();
                    Some((cs.next().unwrap(), cs.next().unwrap()))
                }
            })
            .collect::<Vec<(char, char)>>();
        if chars.is_empty() {
            return Err(ErrorKind::ModifierKindParseError("specified empty chars.".to_string()));
        }
        let chars = chars.into_iter().collect::<HashMap<char, char>>();
        Ok(Self::from_map(chars))
    }
}

impl From<CharacterConverter> for ModifierKind {
    fn from(m: CharacterConverter) -> ModifierKind {
        ModifierKind::CharacterConverter(m)
    }
}

impl CharacterConverter {
    pub fn from_tables(tables: Vec<Table>) -> Self {
        let mut items_map = HashMap::new();
        for table in tables.iter() {
            for item in table.items().iter() {
                items_map.insert(
                    item.from.chars().next().unwrap(),
                    item.to.chars().next().unwrap(),
                );
            }
        }
        Self { items_map }
    }

    pub fn from_map(items_map: HashMap<char, char>) -> Self {
        Self { items_map }
    }
}

/// character_converter!{'D' => 'd', 'E' => 'e'};
#[macro_export]
macro_rules! character_converter {
    ($($from:expr => $to:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($from, $to);
        )*
        crate::chanoma::modifier::character_converter::CharacterConverter::from_map(map)
    }};
}
