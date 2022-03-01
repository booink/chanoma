use super::error::Error;
use super::file::*;
use super::file_extension::FileExtension;
use super::modifier::CharacterConverter;
use super::modifier_kind::ModifierKind;
use anyhow::bail;
use std::env;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

pub struct Configuration {
    debug: bool,
}

impl Configuration {
    const ENV_NAME: &'static str = "CHANOMARC";
    const FILE_NAMES: [&'static str; 2] = ["chanomarc", ".chanomarc"];

    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    fn file_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        // 環境変数が指定されていたら、そのパスだけを返す
        if let Some(path) = file_path_from_env(Self::ENV_NAME) {
            if self.debug {
                log::debug!(
                    "{}: {:?}; (exists: {:?})",
                    Self::ENV_NAME,
                    path,
                    path.exists()
                );
            }
            return valid_file_path(path);
        }

        Ok(exists_file_paths(
            auto_load_dirs(),
            auto_load_paths_from_dir,
        ))
    }

    fn load_items_to_character_converter(
        &self,
        path: &Path,
        ext: &FileExtension,
    ) -> anyhow::Result<CharacterConverter> {
        let table = match ext {
            FileExtension::Csv => table_from_csv_path(path)?,
            FileExtension::Yaml => table_from_yaml_path(path)?,
            //FileExtension::Json => from_json_path(path),
        };
        Ok(CharacterConverter::from_tables(vec![table]))
    }

    fn load_modifiers(
        &self,
        path: &Path,
        ext: &FileExtension,
    ) -> anyhow::Result<Vec<ModifierKind>> {
        let modifier_kinds = match ext {
            FileExtension::Yaml => modifiers_from_yaml_path(path)?,
            //FileExtension::Json => modifiers_from_json_path(path)?,
            _ => Vec::new(),
        };
        Ok(modifier_kinds)
    }

    fn load_file_with_ext(&self, path: &Path) -> anyhow::Result<Vec<ModifierKind>> {
        let ext = FileExtension::from_path(&path)?;
        let character_converter = self.load_items_to_character_converter(path, &ext)?;
        let mut modifiers = self.load_modifiers(path, &ext)?;
        modifiers.push(character_converter.into());
        Ok(modifiers)
    }

    pub fn load(&self) -> anyhow::Result<Vec<ModifierKind>> {
        let file_paths = self.file_paths()?;
        if self.debug {
            log::debug!("load files: {:?}", &file_paths);
        }

        if file_paths.is_empty() {
            return Ok(Vec::new());
        }

        let result = file_paths
            .iter()
            .map(|path| self.load_file_with_ext(path))
            .collect::<anyhow::Result<Vec<Vec<ModifierKind>>>>();

        if let Err(e) = result {
            return Err(e);
        }

        Ok(result
            .unwrap()
            .into_iter()
            .flatten()
            .collect::<Vec<ModifierKind>>())
    }
}

fn file_path_from_env(env_name: &str) -> Option<PathBuf> {
    // 環境変数が指定されていたらパスを返す
    if let Ok(path) = env::var(env_name) {
        // Configuration::ENV_NAME
        return Some(PathBuf::from(path));
    }
    None
}

fn valid_file_path(path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    // 指定したファイルが設定ファイルとして使用可能であればパス情報を返す
    if !path.is_file() {
        bail!(Error::RcFileLoadError(path))
    }
    let extension = String::from(path.extension().unwrap().to_str().unwrap());
    if FileExtension::iter().all(|ext| ext.to_string() != extension) {
        bail!(Error::RcFileLoadError(path))
    }
    Ok(vec![path])
}

fn auto_load_dirs() -> Vec<PathBuf> {
    // ロードするディレクトリのパスを返す
    let mut paths = Vec::new();
    if let Some(home_dir) = dirs::home_dir() {
        let mut dot_config_chanoma = home_dir.clone();
        dot_config_chanoma.push(".config/chanoma");
        paths.push(dot_config_chanoma);
        paths.push(home_dir);
    }
    if let Ok(current_dir) = env::current_dir() {
        paths.push(current_dir);
    }
    paths
}

#[allow(clippy::ptr_arg)]
fn auto_load_paths_from_dir(dir: &PathBuf) -> Vec<PathBuf> {
    // ディレクトリのパスに設定ファイルとして認識可能なパスを生成する
    Configuration::FILE_NAMES
        .iter()
        .flat_map(|file| {
            FileExtension::iter()
                .map(|ext| {
                    let mut path = dir.clone();
                    path.push(file);
                    path.set_extension(ext.to_string());
                    path
                })
                .collect::<Vec<PathBuf>>()
        })
        .collect()
}

fn exists_file_paths<F>(load_dirs: Vec<PathBuf>, expand: F) -> Vec<PathBuf>
where
    F: FnMut(&PathBuf) -> Vec<PathBuf>,
{
    // 存在するパスに絞り込む
    load_dirs
        .iter()
        .flat_map(expand)
        .filter(|path| path.is_file())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod file_path_from_env {
        use super::*;
        use env_test_util::TempEnvVar;

        #[test]
        fn file_path_specified_in_the_environment_variable_can_be_retrieved() {
            // 環境変数で指定したファイルパスが取得できる
            let _variable = TempEnvVar::new(Configuration::ENV_NAME).with("./test"); // 一時的に環境変数に値をセットする
            let path = file_path_from_env(Configuration::ENV_NAME);
            assert_eq!(path, Some(PathBuf::from("./test")));
        }

        #[test]
        fn environment_variable_not_specified() {
            // 環境変数が指定されていない
            assert_eq!(file_path_from_env(Configuration::ENV_NAME), None);
        }
    }

    mod valid_file_path {
        use super::*;

        #[test]
        fn nonexistent_file_path() {
            // 存在しないファイルパスが指定された場合はエラーを返す
            let path = PathBuf::from("/--------a--------");
            assert_eq!(
                valid_file_path(path.clone())
                    .unwrap_err()
                    .downcast::<Error>()
                    .unwrap(),
                Error::RcFileLoadError(path)
            );
        }

        #[test]
        fn existent_file_path_and_no_target_file_extension() {
            // 対応拡張子のファイルが存在しないディレクトリパスが指定された場合はエラーを返す
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
            assert_eq!(
                valid_file_path(path.clone())
                    .unwrap_err()
                    .downcast::<Error>()
                    .unwrap(),
                Error::RcFileLoadError(path)
            );
        }

        #[test]
        fn existent_file_path() {
            // 存在するファイルパスが指定された場合はVecを返す
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/test.csv");
            assert_eq!(valid_file_path(path.clone()).unwrap(), vec![path]);
        }
    }

    mod auto_load_dirs {
        use super::*;

        #[test]
        fn it_works() {
            // 環境変数が指定されていない場合は XDG_CONFIG_HOME, HOME, PWD
            // ディレクトリのパスが取得できる
            let paths = auto_load_dirs();
            let mut dot_config_chanoma = dirs::home_dir().unwrap();
            dot_config_chanoma.push(".config/chanoma");
            assert_eq!(
                paths,
                vec![
                    dot_config_chanoma,
                    dirs::home_dir().unwrap(),
                    env::current_dir().unwrap(),
                ]
            );
        }
    }

    mod auto_load_paths_from_dir {
        use super::*;

        #[test]
        fn it_works() {
            let generated = vec![
                PathBuf::from("./chanomarc.csv"),
                PathBuf::from("./chanomarc.yaml"),
                PathBuf::from("./.chanomarc.csv"),
                PathBuf::from("./.chanomarc.yaml"),
            ];
            assert_eq!(auto_load_paths_from_dir(&PathBuf::from("./")), generated);
        }
    }

    mod exists_file_paths {
        use super::*;

        #[allow(clippy::ptr_arg)]
        fn dummy_paths(dir: &PathBuf) -> Vec<PathBuf> {
            let mut path = dir.clone();
            path.push("test.csv");
            vec![path]
        }

        #[test]
        fn nonexistent_dir_path() {
            // 存在しないディレクトリパスが指定された場合は空のVecが返る
            let path = PathBuf::from("/--------a--------/");
            let result = exists_file_paths(vec![path], dummy_paths);
            assert_eq!(result, Vec::new() as Vec<PathBuf>);
        }

        #[test]
        fn existent_file_path_and_no_target_file_extension() {
            // 対応拡張子のファイルが存在しないディレクトリパスが指定された場合は空のVecが返る
            let result = exists_file_paths(
                vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")],
                dummy_paths,
            );
            assert_eq!(result, Vec::new() as Vec<PathBuf>);
        }

        #[test]
        fn existent_file_path_and_target_file_extension() {
            // 対応ファイルが存在するディレクトリパスが指定された場合はVecが返る
            let result = exists_file_paths(
                vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources")],
                dummy_paths,
            );
            let vec =
                vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/test.csv")];
            assert_eq!(result, vec);
        }
    }
}
