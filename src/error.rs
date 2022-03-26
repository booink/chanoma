use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("csv ファイルの読み込みに失敗しました: {0}")]
    CsvFileLoadError(#[from] csv::Error),
    #[error("yaml ファイルの読み込みに失敗しました: {0}")]
    YamlFileLoadError(#[from] serde_yaml::Error),
    #[error("指定された設定ファイルが存在しません: {0}")]
    RcFileLoadError(std::path::PathBuf),
    #[error("未対応のファイル拡張子が指定されています: {0}")]
    UnsupportedFileExtensionError(String),
    #[error("存在しない Modifier が指定されています: {0}")]
    ModifierKindParseError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
