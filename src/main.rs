//use chanoma::{chanoma::table::Origin, chanoma::PreparedTable, Chanoma};
use chanoma::Chanoma;
use clap::Clap;
use std::io;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "booink <booink.work@gmail.com>"
)]
struct Opts {
    input: Option<String>,
    #[clap(short, long)]
    debug: bool,
//    #[clap(long)]
    //print_table: bool,
    #[clap(short, long)]
    preset: bool,
    #[clap(short, long)]
    neologdn: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
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
            /*
            if opts.print_table {
                print_table(&prepared_table);
            }
            if opts.debug {
                println!("{:?}", result.positions);
            }
            */
            println!("{}", result);
        }
        None => {
            /*
            if opts.print_table {
                print_table(&prepared_table);
            } else {
                */
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let result =
                            chanoma.normalize(input.trim_end_matches('\n'));
                        /*
                        if opts.debug {
                            println!("{:?}", result.positions);
                        }
                        */
                        println!("{}", result);
                    }
                    Err(error) => println!("error: {}", error),
                }
            //}
        }
    };
}

/*
fn print_table(prepared_table: &PreparedTable) {
    for origin_item in prepared_table.items.iter() {
        match &origin_item.origin {
            Origin::New => println!("{} -> {}", origin_item.item.from, origin_item.item.to),
            Origin::ConfigurationFile(path) => println!(
                "{} -> {};\t({})",
                origin_item.item.from, origin_item.item.to, path
            ),
        }
    }
}
*/
