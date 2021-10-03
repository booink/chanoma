use std::cmp::Ordering;

pub type TableItemId = usize;

#[derive(Clone, Debug, Eq)]
pub struct Position {
    pub from: String,
    pub to: String,
    pub index: usize,
    pub length: usize,
    pub utf8_index: usize,
    pub utf8_length: usize,
}

impl Position {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        index: usize,
        utf8_index: usize,
        length: usize,
        utf8_length: usize,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            index,
            utf8_index,
            length,
            utf8_length,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
