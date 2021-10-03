use std::fs;
use std::path::Path;

use super::characters_set::ALL;
use super::corr::{Corr, Correspondence, Item, Synthesized};
use super::file;

#[derive(Clone, Debug)]
pub enum Origin {
    New,
    ConfigurationFile(String),
}

#[derive(Clone)]
pub struct Table {
    corr: Correspondence<Synthesized>,
    pub origin: Origin,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            corr: Correspondence::new(Synthesized::new(Vec::new())),
            origin: Origin::New,
        }
    }

    pub fn from_corr(corr: Correspondence<Synthesized>, origin: Origin) -> Self {
        Self { corr, origin }
    }

    pub fn add<T: Corr>(&mut self, corr: &Correspondence<T>) {
        self.corr += corr;
    }

    pub fn sub<T: Corr>(&mut self, corr: &Correspondence<T>) {
        self.corr -= corr;
    }

    pub fn corr(&self) -> Correspondence<Synthesized> {
        self.corr.clone()
    }

    pub fn items(&self) -> Vec<Item> {
        self.corr.items()
    }
}

pub struct TableBuilder {
    table: Table,
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TableBuilder {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn preset(&mut self) -> &mut Self {
        self.table.add(&ALL);
        self
    }

    pub fn with_corr<T: Corr>(mut self, corr: &Correspondence<T>) -> Self {
        self.table.add(corr);
        self
    }

    pub fn without_corr<T: Corr>(mut self, corr: &Correspondence<T>) -> Self {
        self.table.sub(corr);
        self
    }

    // TODO: Corr間で矛盾する対応付けが存在したら、エラーにする
    // 例: Item::new("a", "A"), Item::new("a", "B")
    // のように、同じfromから複数のtoが存在する場合など。
    pub fn build(&self) -> &Table {
        &self.table
    }

    pub fn from_yaml_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = fs::canonicalize(&path)?;
        Ok(Self {
            table: file::table_from_yaml_path(path)?,
        })
    }

    pub fn from_csv_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = fs::canonicalize(&path)?;
        Ok(Self {
            table: file::table_from_csv_path(path)?,
        })
    }

    pub fn add_from_csv(&mut self, path: impl AsRef<Path>) -> anyhow::Result<&mut Self> {
        let table = file::table_from_csv_path(path.as_ref())?;
        self.table.add(&table.corr());
        Ok(self)
    }

    pub fn add_from_yaml(&mut self, path: impl AsRef<Path>) -> anyhow::Result<&mut Self> {
        let table = file::table_from_yaml_path(path.as_ref())?;
        self.table.add(&table.corr());
        Ok(self)
    }
}
