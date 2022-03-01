use crate::chanoma::characters_set::prelude::*;
use crate::chanoma::corr::{Correspondence, Unification};
use crate::chanoma::error::Error;
use crate::chanoma::modifier::ModifierFromYamlValue;
use crate::chanoma::modifier::{
    CharacterConverter, CharacterEliminator, ConsecutiveCharacterReducer, DottedSpaceEliminator,
    LigatureTranslator, ModifiedData, Modifier,
};
use crate::chanoma::modifier_kind::ModifierKind;
use crate::chanoma::table::{Origin, Table};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neologdn {
    character_converter: CharacterConverter,
    consecutive: ConsecutiveCharacterReducer,
    space_eliminater: DottedSpaceEliminator,
    character_eliminator: CharacterEliminator,
    ligature_translator: LigatureTranslator,
}

impl Modifier for Neologdn {
    fn modify(&self, input: &str) -> String {
        let text = self.character_converter.modify(input);
        let text = self.consecutive.modify(&text);
        let text = self.space_eliminater.modify(&text);
        let text = self.character_eliminator.modify(&text);
        let text = self.ligature_translator.modify(&text);
        self.space_eliminater.modify(&text)
    }

    fn modify_with_positions(&self, input: &str) -> ModifiedData {
        let result = self.character_converter.modify_with_positions(input);
        let result = self.consecutive.modify_with_positions(&result.text);
        let result = self.space_eliminater.modify_with_positions(&result.text);
        let result = self
            .character_eliminator
            .modify_with_positions(&result.text);
        let result = self.ligature_translator.modify_with_positions(&result.text);
        ModifiedData::new(
            ModifierKind::Neologdn(self.clone()),
            result.text,
            result.positions,
        )
    }
}

impl ModifierFromYamlValue for Neologdn {
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

impl FromStr for Neologdn {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self::new())
        } else {
            Err(Error::ModifierKindParseError(
                "Cannot specify a value.".to_string(),
            ))
        }
    }
}

impl From<Neologdn> for ModifierKind {
    fn from(m: Neologdn) -> ModifierKind {
        ModifierKind::Neologdn(m)
    }
}

impl Default for Neologdn {
    fn default() -> Self {
        Self::new()
    }
}

const KANA_TEN: [(&str, &str); 42] = [
    ("か゛", "が"),
    ("き゛", "ぎ"),
    ("く゛", "ぐ"),
    ("け゛", "げ"),
    ("こ゛", "ご"),
    ("さ゛", "ざ"),
    ("し゛", "じ"),
    ("す゛", "ず"),
    ("せ゛", "ぜ"),
    ("そ゛", "ぞ"),
    ("た゛", "だ"),
    ("ち゛", "ぢ"),
    ("つ゛", "づ"),
    ("て゛", "で"),
    ("と゛", "ど"),
    ("は゛", "ば"),
    ("ひ゛", "び"),
    ("ふ゛", "ぶ"),
    ("へ゛", "べ"),
    ("ほ゛", "ぼ"),
    ("う゛", "ゔ"),
    ("カ゛", "ガ"),
    ("キ゛", "ギ"),
    ("ク゛", "グ"),
    ("ケ゛", "ゲ"),
    ("コ゛", "ゴ"),
    ("サ゛", "ザ"),
    ("シ゛", "ジ"),
    ("ス゛", "ズ"),
    ("セ゛", "ゼ"),
    ("ソ゛", "ゾ"),
    ("タ゛", "ダ"),
    ("チ゛", "ヂ"),
    ("ツ゛", "ヅ"),
    ("テ゛", "デ"),
    ("ト゛", "ド"),
    ("ハ゛", "バ"),
    ("ヒ゛", "ビ"),
    ("フ゛", "ブ"),
    ("ヘ゛", "ベ"),
    ("ホ゛", "ボ"),
    ("ウ゛", "ヴ"),
];

const KANA_MARU: [(&str, &str); 10] = [
    ("ハ゜", "パ"),
    ("ヒ゜", "ピ"),
    ("フ゜", "プ"),
    ("ヘ゜", "ペ"),
    ("ホ゜", "ポ"),
    ("は゜", "ぱ"),
    ("ひ゜", "ぴ"),
    ("ふ゜", "ぷ"),
    ("へ゜", "ぺ"),
    ("ほ゜", "ぽ"),
];

const TILDES: [char; 6] = ['~', '∼', '∾', '〜', '〰', '～'];

// 長音記号
const CHOONPUS: Correspondence<Unification> =
    //Unification::new(&["﹣", "－", "ｰ", "—", "―", "─", "━"], "ー").corr();
    Unification::new(&["﹣", "ｰ", "—", "―", "─", "━"], "ー").corr();
// ハイフン
const HYPHENS: Correspondence<Unification> =
    //Unification::new(&["˗", "֊", "‐", "‑", "‒", "–", "⁃", "⁻", "₋", "−"], "-").corr();
    Unification::new(
        &["˗", "֊", "‐", "‑", "‒", "–", "⁃", "⁻", "₋", "−", "－"],
        "-",
    )
    .corr();

impl Neologdn {
    pub fn new() -> Self {
        let corr = ALL - &EQUAL_SIGN + &CHOONPUS + &HYPHENS;
        let table = Table::from_corr(corr, Origin::New);
        let mut map = HashMap::new();
        for (f, t) in KANA_TEN.iter() {
            map.insert(f.to_string(), t.to_string());
        }
        for (f, t) in KANA_MARU.iter() {
            map.insert(f.to_string(), t.to_string());
        }
        Self {
            character_converter: CharacterConverter::from_tables(vec![table]),
            consecutive: ConsecutiveCharacterReducer::new('ー'),
            space_eliminater: DottedSpaceEliminator::new(),
            character_eliminator: CharacterEliminator::from_chars(TILDES.to_vec()),
            ligature_translator: LigatureTranslator::from_map(map),
        }
    }
}
