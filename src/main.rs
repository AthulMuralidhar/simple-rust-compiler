// remove when done
#![allow(unused_variables)]

mod cli;
mod logger;

use cli::init_cli;
use logger::init_logger;


macro_rules! unwrap_or_exit {
    (f:expr, origin:tt) => {
        match f {
            Ok(a) => a,
            Err(e) => {
                error!("{}: {}", origin, e);
                process::exit(1);
            }
        }
    };
}

fn main() {
    // does not print to std out
    // info!("init ci"); so go with print
    println!("init ci");
    let ci_input = init_cli();

    init_logger(ci_input.verbose);

    // lexer
    println!("init lexer");
    let lexer  = unwrap_or_exit!(Lexer::from_file(&cli_input.input_path));

}


// TODO
// replicate this
// https://www.andrewmin.info/blog/compiler-1/