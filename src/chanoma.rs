use crate::configuration::Configuration;
use crate::modifier::{CharacterConverter, ModifiedRecord, Modifier, Neologdn};
use crate::modifier_kind::ModifierKind;
use crate::table::{Table, TableBuilder};
use crate::ChanomaResult;
use log::log_enabled;
use std::path::Path;

pub struct Chanoma {
    modifiers: Vec<ModifierKind>,
}

impl Default for Chanoma {
    fn default() -> Self {
        Self::new()
    }
}

impl Chanoma {
    /// [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::new();
    /// ```
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    /// 特定のファイルパスに配置された設定ファイルを読み込んで [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::load_rc();
    /// ```
    pub fn load_rc() -> anyhow::Result<Self> {
        Ok(Self {
            modifiers: Configuration.load()?,
        })
    }

    /// ファイルパスの設定ファイルを読み込んで [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::from_config_file("/path/to/config.csv");
    /// ```
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> ChanomaResult<Self> {
        Ok(Self {
            modifiers: Configuration::from_path(path.as_ref())?,
        })
    }

    /// プリセットされた設定を使用するように [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::preset();
    /// ```
    pub fn preset() -> Self {
        Self {
            modifiers: vec![Self::all_characters_set()],
        }
    }

    /// プリセットされた設定を使用します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let mut chanoma = Chanoma::new();
    /// chanoma.use_preset();
    /// ```
    pub fn use_preset(&mut self) -> &mut Self {
        self.modifiers.push(Self::all_characters_set());
        self
    }

    /// [neologdn](https://github.com/ikegami-yukino/neologdn) の同様の結果になる設定を使用するように [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::neologdn();
    /// ```
    pub fn neologdn() -> Self {
        Self {
            modifiers: vec![Neologdn::new().into()],
        }
    }

    /// [neologdn](https://github.com/ikegami-yukino/neologdn) の同様の結果になる設定を使用します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let mut chanoma = Chanoma::new();
    /// chanoma.use_neologdn();
    /// ```
    pub fn use_neologdn(&mut self) -> &mut Self {
        self.modifiers.push(Neologdn::new().into());
        self
    }

    /// 正規化処理を実行します。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::preset();
    /// println!("{}", chanoma.normalize("ｵﾊﾖｳｺﾞｻﾞｲﾏｽ"));
    /// ```
    pub fn normalize(&self, text: &str) -> String {
        if !log_enabled!(log::Level::Debug) {
            self.modifiers
                .iter()
                .fold(text.to_string(), |acc, x| x.modify(&acc))
        } else {
            self.normalize_with_positions(text).text().to_string()
        }
    }

    /// 正規化処理を実行します。
    /// 処理の途中経過をスタックして保持しています。
    ///
    /// ```
    /// use chanoma::Chanoma;
    ///
    /// let chanoma = Chanoma::preset();
    /// println!("{:?}", chanoma.normalize_with_positions("ｵﾊﾖｳｺﾞｻﾞｲﾏｽ"));
    /// ```
    pub fn normalize_with_positions(&self, text: &str) -> ModifiedResult {
        let mut v = Vec::new();
        let mut text = text.to_string();
        for m in self.modifiers.iter() {
            let result = m.modify_with_positions(&text);
            if log_enabled!(log::Level::Debug) {
                log::debug!("{:?}", &result);
            }
            v.push(result.clone());
            text = result.text;
        }
        ModifiedResult::new(v)
    }

    /// 一文字から一文字の置換テーブルを定義した [Table] 構造体を使用するように [Chanoma] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::{Chanoma, TableBuilder};
    ///
    /// let table = TableBuilder::new().preset().build();
    /// let chanoma = Chanoma::from_table(table);
    /// ```
    pub fn from_table(table: Table) -> Self {
        Self {
            modifiers: vec![CharacterConverter::from_tables(vec![table]).into()],
        }
    }

    /// 一文字から一文字の置換テーブルを定義した [Table] 構造体を使用します。
    ///
    /// ```
    /// use chanoma::{Chanoma, TableBuilder};
    ///
    /// let table = TableBuilder::new().preset().build();
    /// let mut chanoma = Chanoma::new();
    /// chanoma.add_table(table);
    /// ```
    pub fn add_table(&mut self, table: Table) -> &mut Self {
        self.modifiers
            .push(CharacterConverter::from_tables(vec![table]).into());
        self
    }

    fn all_characters_set() -> ModifierKind {
        CharacterConverter::from_tables(vec![TableBuilder::new().preset().build()]).into()
    }
}

#[derive(Debug)]
pub struct ModifiedResult {
    records: Vec<ModifiedRecord>,
}

impl ModifiedResult {
    pub fn new(data: Vec<ModifiedRecord>) -> Self {
        Self { records: data }
    }

    pub fn text(&self) -> &str {
        &self.records.last().unwrap().text
    }

    pub fn modified_records(&self) -> &Vec<ModifiedRecord> {
        &self.records
    }
}
