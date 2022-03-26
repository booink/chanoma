use crate::error::Error;
use crate::modifier::character_converter::CharacterConverter;
use crate::modifier::character_eliminator::CharacterEliminator;
use crate::modifier::consecutive_character_reducer::ConsecutiveCharacterReducer;
use crate::modifier::dotted_space_eliminator::DottedSpaceEliminator;
use crate::modifier::ligature_translator::LigatureTranslator;
use crate::modifier::neologdn::Neologdn;
use crate::modifier::trim::Trim;
use crate::modifier::ModifierFromYamlValue;
use crate::modifier::{ModifiedRecord, Modifier};
use crate::ChanomaResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierKind {
    /// 一文字を別の一文字に置換します
    CharacterConverter(CharacterConverter),
    /// 文字を削除します
    CharacterEliminator(CharacterEliminator),
    /// 連続する同じ文字を一つにします
    ConsecutiveCharacterReducer(ConsecutiveCharacterReducer),
    /// 『「半角英数字」と「半角英数字」の間の半角スペース』以外の半角スペースを削除します
    DottedSpaceEliminator(DottedSpaceEliminator),
    /// 合字をくっつけて一文字にします
    LigatureTranslator(LigatureTranslator),
    /// 先頭と末尾の半角スペースを削除します
    Trim(Trim),
    /// neologdn
    Neologdn(Neologdn),
}

impl FromStr for ModifierKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "neologdn" => Ok(Self::Neologdn(Neologdn::new())),
            "trim" => Ok(Self::Trim(Trim::new())),
            "dotted_space_eliminator" => {
                Ok(Self::DottedSpaceEliminator(DottedSpaceEliminator::new()))
            }
            _ => {
                if s.starts_with("consecutive_character_reducer(") {
                    // ex. consecutive_character_reducer(ー)
                    let s = s
                        .strip_prefix("consecutive_character_reducer(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap();
                    let consecutive_character_reducer = ConsecutiveCharacterReducer::from_str(s)?;
                    Ok(Self::ConsecutiveCharacterReducer(
                        consecutive_character_reducer,
                    ))
                } else if s.starts_with("ligature_translator(") {
                    // ligature_translator(ハ゜,パ)
                    let s = s
                        .strip_prefix("ligature_translator(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap();
                    let ligature_translator = LigatureTranslator::from_str(s)?;
                    Ok(Self::LigatureTranslator(ligature_translator))
                } else if s.starts_with("character_eliminator(") {
                    // character_eliminator(~, ∼, ∾, 〜, 〰, ～)
                    let s = s
                        .strip_prefix("character_eliminator(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap();
                    let character_eliminator = CharacterEliminator::from_str(s)?;
                    Ok(Self::CharacterEliminator(character_eliminator))
                } else if s.starts_with("character_converter(") {
                    // character_converter(Ａ: A, Ｂ: B, Ｃ: C, Ｄ: D, Ｅ: E)
                    let s = s
                        .strip_prefix("character_converter(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap();
                    let character_converter = CharacterConverter::from_str(s)?;
                    Ok(Self::CharacterConverter(character_converter))
                } else {
                    Err(Error::ModifierKindParseError("parse error.".to_string()))
                }
            }
        }
    }
}

impl Modifier for ModifierKind {
    fn modify(&self, input: &str) -> String {
        match self {
            Self::CharacterConverter(x) => x.modify(input),
            Self::CharacterEliminator(x) => x.modify(input),
            Self::ConsecutiveCharacterReducer(x) => x.modify(input),
            Self::DottedSpaceEliminator(x) => x.modify(input),
            Self::LigatureTranslator(x) => x.modify(input),
            Self::Trim(x) => x.modify(input),
            Self::Neologdn(x) => x.modify(input),
        }
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedRecord {
        match self {
            Self::CharacterConverter(x) => x.modify_with_positions(input),
            Self::CharacterEliminator(x) => x.modify_with_positions(input),
            Self::ConsecutiveCharacterReducer(x) => x.modify_with_positions(input),
            Self::DottedSpaceEliminator(x) => x.modify_with_positions(input),
            Self::LigatureTranslator(x) => x.modify_with_positions(input),
            Self::Trim(x) => x.modify_with_positions(input),
            Self::Neologdn(x) => x.modify_with_positions(input),
        }
    }
}

impl ModifierKind {
    pub fn from_yaml_key_value(
        key: &serde_yaml::Value,
        value: &serde_yaml::Value,
    ) -> ChanomaResult<Self> {
        match key.as_str().unwrap() {
            "character_converter" => Ok(Self::CharacterConverter(
                CharacterConverter::from_yaml_value(value)?,
            )),
            "character_eliminator" => Ok(Self::CharacterEliminator(
                CharacterEliminator::from_yaml_value(value)?,
            )),
            "consecutive_character_reducer" => Ok(Self::ConsecutiveCharacterReducer(
                ConsecutiveCharacterReducer::from_yaml_value(value)?,
            )),
            "ligature_translator" => Ok(Self::LigatureTranslator(
                LigatureTranslator::from_yaml_value(value)?,
            )),
            "dotted_space_eliminator" => Ok(Self::DottedSpaceEliminator(
                DottedSpaceEliminator::from_yaml_value(value)?,
            )),
            "trim" => Ok(Self::Trim(Trim::from_yaml_value(value)?)),
            "neologdn" => Ok(Self::Neologdn(Neologdn::from_yaml_value(value)?)),
            _ => Err(Error::ModifierKindParseError(format!(
                "specified key {} does not exists.",
                key.as_str().unwrap()
            ))),
        }
    }
}
