use crate::chanoma::error::Error;
use crate::chanoma::modifier::character_converter::CharacterConverter;
use crate::chanoma::modifier::character_eliminator::CharacterEliminator;
use crate::chanoma::modifier::consecutive_character_reducer::ConsecutiveCharacterReducer;
use crate::chanoma::modifier::dotted_space_eliminator::DottedSpaceEliminator;
use crate::chanoma::modifier::ligature_translator::LigatureTranslator;
use crate::chanoma::modifier::neologdn::Neologdn;
use crate::chanoma::modifier::trim::Trim;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::modifier::{ModifiedData, Modifier};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierKind {
    CharacterConverter(CharacterConverter),
    CharacterEliminator(CharacterEliminator),
    ConsecutiveCharacterReducer(ConsecutiveCharacterReducer),
    DottedSpaceEliminator(DottedSpaceEliminator),
    LigatureTranslator(LigatureTranslator),
    Trim(Trim),
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

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
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
    ) -> anyhow::Result<Self> {
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
            ))
            .into()),
        }
    }
}

/*
pub fn modifier_key_map(key: &str) -> anyhow::Result<XModifier<Box<dyn Modifier>>> {
    let kind = ModifierKind::from_str(key);
    if kind.is_err() {
        return Err(anyhow::anyhow!("ParseModifierKindError"));
    }
    Ok(match kind.unwrap() {
        ModifierKind::Neologdn => XModifier::new(Box::new(Neologdn::new())),
        ModifierKind::Trim => XModifier::new(Box::new(Trim {})),
        ModifierKind::DottedSpaceEliminator => {
            XModifier::new(Box::new(DottedSpaceEliminator::new()))
        }
        ModifierKind::LigatureTranslator(map) => {
            XModifier::new(Box::new(LigatureTranslator::from_map(map)))
        }
        ModifierKind::ConsecutiveCharacterReducer(c) => {
            XModifier::new(Box::new(ConsecutiveCharacterReducer::new(c)))
        }
        ModifierKind::CharacterEliminator(chars) => {
            XModifier::new(Box::new(CharacterEliminator::from_chars(chars)))
        }
        ModifierKind::CharacterConverter(map) => {
            XModifier::new(Box::new(CharacterConverter::from_map(map)))
        }
    })
}
*/
