use super::error::ErrorKind;
use std::str::FromStr;
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum FileExtension {
    Csv,
    Yaml,
    //Json,
}

impl FromStr for FileExtension {
    type Err = ErrorKind;
    fn from_str(ext: &str) -> Result<Self, ErrorKind> {
        match ext {
            ".csv" | "csv" => Ok(Self::Csv),
            ".yaml" | "yaml" | ".yml" | "yml" => Ok(Self::Yaml),
            //".json" | "json" => Ok(Self::Json),
            _ => Err(ErrorKind::UnsupportedFileExtensionError(ext.to_string())),
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ErrorKind> {
        Self::from_str(path.as_ref().extension().unwrap().to_str().unwrap())
    }
}
