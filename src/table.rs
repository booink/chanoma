//! 置換テーブルを定義するモジュールです。

use super::characters_set::ALL;
use super::corr::{Corr, Correspondence, Item, Synthesized};
use super::file;
use crate::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// 一文字から一文字の置換テーブルを定義している場所を指定する列挙型です。
#[derive(Clone, Debug)]
pub enum Origin {
    /// 手動で新規作成する際に指定します。
    New,
    /// 設定ファイルから読み込んだ際に指定します。
    ConfigurationFile(PathBuf),
}

/// 一文字から一文字の置換テーブルを定義する構造体です。
#[derive(Clone)]
pub struct Table {
    /// 一文字から一文字の置換テーブルを指定します。
    corr: Correspondence<Synthesized>,
    /// 一文字から一文字の置換テーブルを定義している場所を指定します。
    pub origin: Origin,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// [Table] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Table;
    ///
    /// let table = Table::new();
    /// ```
    pub fn new() -> Self {
        Self {
            corr: Correspondence::default(),
            origin: Origin::New,
        }
    }

    /// [TableBuilder] を初期化して返します。
    ///
    /// ```
    /// use chanoma::Table;
    ///
    /// let table = Table::new().builder();
    /// ```
    pub fn builder(self) -> TableBuilder {
        TableBuilder::from_table(self)
    }

    /// 置換テーブルに [Correspondence] を登録します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, Table, table::Origin};
    ///
    /// let corr: Correspondence<Synthesized> = Correspondence::default();
    /// let table = Table::from_corr(corr, Origin::New);
    /// ```
    pub fn from_corr(corr: Correspondence<Synthesized>, origin: Origin) -> Self {
        Self { corr, origin }
    }

    /// 置換テーブルに [Correspondence] を追加します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, Table};
    ///
    /// let mut table = Table::new();
    /// let corr: Correspondence<Synthesized> = Correspondence::default();
    /// table.add(&corr);
    /// ```
    pub fn add<T: Corr>(&mut self, corr: &Correspondence<T>) {
        self.corr += corr;
    }

    /// 置換テーブルに登録されている [Correspondence] を削除します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, Table};
    ///
    /// let mut table = Table::new();
    /// let corr: Correspondence<Synthesized> = Correspondence::default();
    /// table.remove(&corr);
    /// ```
    pub fn remove<T: Corr>(&mut self, corr: &Correspondence<T>) {
        self.corr -= corr;
    }

    /// 置換テーブルに登録されている [Correspondence] を返します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, Table};
    ///
    /// let table = Table::new();
    /// println!("{:?}", table.corr());
    /// ```
    pub fn corr(&self) -> Correspondence<Synthesized> {
        self.corr.clone()
    }

    /// 置換テーブルに登録されている [Item] を返します。
    ///
    /// ```
    /// use chanoma::Table;
    ///
    /// let table = Table::new();
    /// println!("{:?}", table.items());
    /// ```
    pub fn items(&self) -> Vec<Item> {
        self.corr.items()
    }
}

/// 一文字から一文字の置換テーブルを構築するための構造体です。
pub struct TableBuilder {
    table: Table,
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TableBuilder {
    /// [TableBuilder] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let builder = TableBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    /// [Table] 構造体から [TableBuilder] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::{Table, TableBuilder};
    ///
    /// let table = Table::new();
    /// let builder = TableBuilder::from_table(table);
    /// ```
    pub fn from_table(table: Table) -> Self {
        Self { table }
    }

    /// 置換テーブルに preset を登録します。
    ///
    /// ```
    /// use chanoma::{Table, TableBuilder};
    ///
    /// let builder = TableBuilder::new().preset();
    /// ```
    pub fn preset(mut self) -> Self {
        self.table.add(&ALL);
        self
    }

    /// 置換テーブルに [Correspondence] を追加します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, TableBuilder};
    ///
    /// let corr: Correspondence<Synthesized> = Correspondence::default();
    /// let builder = TableBuilder::new().add_corr(&corr);
    /// ```
    pub fn add_corr<T: Corr>(mut self, corr: &Correspondence<T>) -> Self {
        self.table.add(corr);
        self
    }

    /// 置換テーブルから [Correspondence] を削除します。
    ///
    /// ```
    /// use chanoma::{Correspondence, Synthesized, TableBuilder};
    ///
    /// let corr: Correspondence<Synthesized> = Correspondence::default();
    /// let builder = TableBuilder::new().remove_corr(&corr);
    /// ```
    pub fn remove_corr<T: Corr>(mut self, corr: &Correspondence<T>) -> Self {
        self.table.remove(corr);
        self
    }

    /// 構築した [Table] を返します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let builder = TableBuilder::new().preset();
    /// let table = builder.build();
    /// ```
    // TODO: Corr間で矛盾する対応付けが存在したら、エラーにする
    // 例: Item::new("a", "A"), Item::new("a", "B")
    // のように、同じfromから複数のtoが存在する場合など。
    pub fn build(self) -> Table {
        self.table
    }

    /// yaml ファイルのパスを指定して [TableBuilder] を初期化します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let builder = TableBuilder::from_yaml_path("./table.yaml");
    /// ```
    pub fn from_yaml_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = fs::canonicalize(&path)?;
        Ok(Self {
            table: file::table_from_yaml_path(path)?,
        })
    }

    /// csv ファイルのパスを指定して [TableBuilder] を初期化します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let builder = TableBuilder::from_csv_path("./table.csv");
    /// ```
    pub fn from_csv_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = fs::canonicalize(&path)?;
        Ok(Self {
            table: file::table_from_csv_path(path)?,
        })
    }

    /// csv ファイルのパスを指定して置換テーブルを追加します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let mut builder = TableBuilder::new();
    /// builder.add_from_csv("./table.csv");
    /// ```
    pub fn add_from_csv(&mut self, path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        let table = file::table_from_csv_path(path.as_ref())?;
        self.table.add(&table.corr());
        Ok(self)
    }

    /// yaml ファイルのパスを指定して置換テーブルを追加します。
    ///
    /// ```
    /// use chanoma::TableBuilder;
    ///
    /// let mut builder = TableBuilder::new();
    /// builder.add_from_yaml("./table.yaml");
    /// ```
    pub fn add_from_yaml(&mut self, path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        let table = file::table_from_yaml_path(path.as_ref())?;
        self.table.add(&table.corr());
        Ok(self)
    }
}
