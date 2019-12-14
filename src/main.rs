extern crate clap;
extern crate pest;
extern crate regex;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use clap::{App, Arg, ArgMatches, SubCommand};

mod util;
mod parser;

use parser::parse_file;

fn main() {
    let matches = App::new("Mdlroptimizer")
        .version("0.1")
        .about("Tool for optimizing mdl models.")
        .author("Nebula Venus (Github)")
        .arg(Arg::with_name("dir")
            .help("Process all files in directory")
            .takes_value(true)
            .short("p")
            .short("parse-all"))
        .get_matches();

    if let Some(dir) = matches.value_of("dir") {
        // Logic
    }
}
