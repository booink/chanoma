use serde::Serialize;

/// 置換前と置換後を保持する構造体
#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize)]
pub struct Item {
    /// 置換前
    pub from: String,
    /// 置換後
    pub to: String,
}

impl Item {
    /// [Item] 構造体を初期化します。
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}
