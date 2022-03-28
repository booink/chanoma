//! 対応している設定ファイルの拡張子を扱うモジュールです。

use crate::error::Error;
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumIter;

/// 対応済みの拡張子の列挙型です。
#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum FileExtension {
    Csv,
    Yaml,
    //Json,
}

impl FromStr for FileExtension {
    type Err = Error;
    fn from_str(ext: &str) -> Result<Self, Self::Err> {
        match ext {
            ".csv" | "csv" => Ok(Self::Csv),
            ".yaml" | "yaml" | ".yml" | "yml" => Ok(Self::Yaml),
            //".json" | "json" => Ok(Self::Json),
            _ => Err(Error::UnsupportedFileExtensionError(ext.to_string())),
        }
    }
}

impl ToString for FileExtension {
    fn to_string(&self) -> String {
        match self {
            Self::Csv => "csv".to_string(),
            Self::Yaml => "yaml".to_string(),
            //Self::Json => "json".to_string(),
        }
    }
}

impl FileExtension {
    /// ファイルパスから拡張子を特定します。
    ///
    /// ```
    /// use chanoma::file_extension::FileExtension;
    ///
    /// let ext = FileExtension::from_path("/path/to/foo.csv").unwrap();
    /// assert_eq!(ext, FileExtension::Csv);
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let extension = match path.extension() {
            Some(ext) => Ok(ext),
            None => Err(Error::FileExtensionNotFoundInPath(path.to_path_buf())),
        }?;
        let ext_str = match extension.to_str() {
            Some(s) => Ok(s),
            None => Err(Error::FileExtensionCanNotConvertToStr(path.to_path_buf())),
        }?;
        Self::from_str(ext_str)
    }
}
