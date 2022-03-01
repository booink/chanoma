use super::error::Error;
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumIter;

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::from_str(path.as_ref().extension().unwrap().to_str().unwrap())
    }
}
