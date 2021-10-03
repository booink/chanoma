use super::corr::{Corr, Correspondence, Item, Synthesized};
use super::table::{Origin, Table};
use serde::Serialize;
use std::collections::BTreeMap;
use super::modifier_kind::ModifierKind;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn table_from_csv_path(path: impl AsRef<Path>) -> anyhow::Result<Table> {
    let path = path.as_ref();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_path(path)?;
    let mut items = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let to = record.iter().last().unwrap();
        let last_index = record.len() - 1;
        for i in 0..last_index {
            items.push(Item::new(&record[i], to));
        }
    }
    Ok(Table::from_corr(
        Correspondence::new(Synthesized::new(items)),
        Origin::ConfigurationFile(path.to_str().unwrap().to_string()),
    ))
}

pub fn to_csv_writer<T: Corr, W: Write>(corr: &Correspondence<T>, writer: W) -> csv::Writer<W> {
    let mut wtr = csv::WriterBuilder::new().flexible(true).from_writer(writer);
    for (to, froms) in Synthesized::from(corr).item_map().iter_mut() {
        froms.push(&to);
        wtr.write_record(froms).unwrap();
    }
    wtr
}

pub fn table_from_yaml_path(path: impl AsRef<Path>) -> anyhow::Result<Table> {
    let path = path.as_ref();
    let data: serde_yaml::Value = serde_yaml::from_reader(File::open(path)?)?;
    let mut items = Vec::new();
    if let Some(map) = data.get("items") {
        for (key, value) in map.as_mapping().unwrap().iter() {
            if value.is_sequence() {
                for v in value.as_sequence().unwrap().iter() {
                    items.push(Item::new(v.as_str().unwrap(), key.as_str().unwrap()));
                }
            } else {
                items.push(Item::new(value.as_str().unwrap(), key.as_str().unwrap()));
            }
        }
    }
    Ok(Table::from_corr(
        Correspondence::new(Synthesized::new(items)),
        Origin::ConfigurationFile(path.to_str().unwrap().to_string()),
    ))
}

pub fn modifiers_from_yaml_path(path: impl AsRef<Path>) -> anyhow::Result<Vec<ModifierKind>> {
    let path = path.as_ref();
    let data: serde_yaml::Value = serde_yaml::from_reader(File::open(path)?)?;
    let mut modifiers = Vec::new();
    if let Some(map) = data.get("modifiers") {
        for (key, value) in map.as_mapping().unwrap().iter() {
            modifiers.push(ModifierKind::from_yaml_key_value(key, value)?);
        }
    }
    Ok(modifiers)
}

/*
pub fn table_from_json_path(path: impl AsRef<Path>) -> anyhow::Result<Table> {
    table_from_yaml_path(path)
}
*/

#[derive(Serialize)]
pub struct SerializedCorr {
    items: BTreeMap<String, Vec<String>>,
}

impl<T> From<&Correspondence<T>> for SerializedCorr
where
    T: Corr,
{
    fn from(corr: &Correspondence<T>) -> Self {
        let mut b = BTreeMap::new();
        for (k, v) in Synthesized::from(corr).item_map().iter() {
            b.insert(
                k.to_string(),
                v.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            );
        }
        SerializedCorr { items: b }
    }
}

pub fn to_serialized_corr<T: Corr>(corr: &Correspondence<T>) -> SerializedCorr {
    SerializedCorr::from(corr)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    fn items() -> Vec<Item> {
        vec![
            Item::new("a", "A"),
            Item::new("b", "B"),
            Item::new("c", "C"),
            Item::new("sea", "C"),
            Item::new("see", "C"),
        ]
    }

    mod table_from_csv_path {
        use super::*;

        #[test]
        fn success_to_load_csv() {
            let result = table_from_csv_path(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources/test.csv"),
            );
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().corr(),
                Correspondence::new(Synthesized::new(items()))
            );
        }

        #[test]
        fn fail_to_load_file() {
            let result = table_from_csv_path("./nonexistent.file");
            assert!(result.is_err());
        }
    }

    mod to_csv_writer {
        use super::*;

        #[test]
        fn success_to_return_csv_writer() {
            let corr = Correspondence::new(Synthesized::new(items()));
            let mut _v = Vec::new();
            let wtr = to_csv_writer(&corr, _v);
            let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
            assert_eq!(data, "a,A\nb,B\nc,sea,see,C\n");
        }
    }

    mod table_from_yaml_path {
        use super::*;

        #[test]
        fn success_to_load_yaml() {
            let result = table_from_yaml_path(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources/test.yaml"),
            );
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().corr(),
                Correspondence::new(Synthesized::new(items()))
            );
        }

        #[test]
        fn fail_to_load_file() {
            let result = table_from_yaml_path("./nonexistent.file");
            assert!(result.is_err());
        }
    }

    mod modifiers_from_yaml_path {
        use super::*;
        use crate::chanoma::modifier_kind::ModifierKind;
        use crate::chanoma::modifier::dotted_space_eliminator::DottedSpaceEliminator;
        use crate::chanoma::modifier::trim::Trim;
        use crate::chanoma::modifier::neologdn::Neologdn;
        use crate::{character_converter, character_eliminator, consecutive_character_reducer, ligature_translator};

        #[test]
        fn success_to_load_yaml() {
            let result = modifiers_from_yaml_path(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resources/test.yaml"),
            );
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap(),
                vec![
                    ModifierKind::CharacterConverter(character_converter!{'D' => 'd', 'E' => 'e'}),
                    ModifierKind::CharacterEliminator(character_eliminator!['~', '∼', '∾', '〜', '〰', '～']),
                    ModifierKind::ConsecutiveCharacterReducer(consecutive_character_reducer!('ー')),
                    ModifierKind::DottedSpaceEliminator(DottedSpaceEliminator::new()),
                    ModifierKind::Trim(Trim::new()),
                    ModifierKind::LigatureTranslator(ligature_translator!{"ハ゜" => 'パ'}),
                    ModifierKind::Neologdn(Neologdn::new()),
                ],
            );
        }
    }
}
