pub mod chanoma;
pub mod characters_set;
mod configuration;
pub mod corr;
pub mod error;
pub mod file;
pub mod file_extension;
mod modifier;
mod modifier_kind;
pub mod position;
pub mod table;

pub use crate::chanoma::Chanoma;
pub use corr::{Corr, Correspondence, Item, Synthesized};
pub use position::{Position, TableItemId};
pub use table::{Table, TableBuilder};

pub type ChanomaResult<T> = Result<T, error::Error>;
