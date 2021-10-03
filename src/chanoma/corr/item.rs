use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize)]
pub struct Item {
    pub from: String,
    pub to: String,
}

impl Item {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}
