use serde::Serialize;

/// 置換前と置換後の文字列を保持する構造体です。
#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize)]
pub struct Item {
    /// 置換前の文字列を保持します。
    pub from: String,
    /// 置換後の文字列を保持します。
    pub to: String,
}

impl Item {
    /// [Item] 構造体を初期化します。
    ///
    /// ```
    /// use chanoma::corr::Item;
    ///
    /// let item = Item::new("a", "b");
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}
