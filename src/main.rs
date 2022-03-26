//use chanoma::{chanoma::table::Origin, chanoma::PreparedTable, Chanoma};
use chanoma::Chanoma;
use clap::Parser;
use std::io;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "booink <booink.work@gmail.com>")]
struct Opts {
    input: Option<String>,
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    preset: bool,
    #[clap(short, long)]
    neologdn: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let log_level = if opts.debug { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", log_level);
    env_logger::init();

    let chanoma = Chanoma::load_rc(opts.debug);
    if let Err(e) = chanoma {
        panic!("{:?}", e);
    }

    let mut chanoma = chanoma.unwrap();
    if opts.preset {
        chanoma.use_preset();
    }
    if opts.neologdn {
        chanoma.use_neologdn();
    }
    match opts.input {
        Some(input) => {
            let result = chanoma.normalize(&input);
            println!("{}", result);
        }
        None => {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let result = chanoma.normalize(input.trim_end_matches('\n'));
                    println!("{}", result);
                }
                Err(error) => println!("error: {}", error),
            }
            //}
        }
    };
}
