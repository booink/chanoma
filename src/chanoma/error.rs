use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("CSVFileLoadError: {0}")]
    CsvFileLoadError(String),
    #[error("YamlFileLoadError: {0}")]
    YamlFileLoadError(String),
    #[error("RcFileLoadError: {0}")]
    RcFileLoadError(PathBuf),
    #[error("UnsupportedFileExtension: {0}")]
    UnsupportedFileExtensionError(String),
    #[error("ModifierKindParseError: {0}")]
    ModifierKindParseError(String),
}
