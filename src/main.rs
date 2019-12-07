extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("MdlROptimizer")
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
