use chanoma::{
    file::{to_csv_writer, to_serialized_corr},
    Table, TableBuilder,
};
use clap::Parser;
use std::error::Error;

enum Format {
    Csv,
    Yaml,
}

impl Format {
    pub fn from_str(f: &str) -> Self {
        match f {
            "yaml" => Self::Yaml,
            _ => Self::Csv,
        }
    }

    pub fn output(&self, table: &Table) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Csv => {
                let wtr = to_csv_writer(&table.corr(), vec![]);
                println!("{}", String::from_utf8(wtr.into_inner()?)?);
            }
            Self::Yaml => {
                println!(
                    "{}",
                    serde_yaml::to_string(&to_serialized_corr(&table.corr()))?
                );
            }
        }
        Ok(())
    }
}

#[derive(Parser)]
#[clap(version = "0.1.0", author = "booink <booink.work@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "csv")]
    format: String,
}

fn main() {
    let table = TableBuilder::new().preset();
    let opts: Opts = Opts::parse();
    Format::from_str(&opts.format)
        .output(&table.build())
        .expect("error.");
}
