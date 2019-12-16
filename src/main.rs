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
mod model;
mod optimizer;

use parser::parse_file;
use optimizer::optimize_model;
use crate::util::{remove_redundant_lines, replace_values_at_spans};

pub fn parse_optimize_model() {
    let (file, model) = parse_file("././testfiles/ChaosWarrior_unopt.mdl");
    let (redundant_lines, ts, rs) = optimize_model(model);
    let (f1, d1) = remove_redundant_lines(file, redundant_lines);
    //let (f2, d2) = replace_values_at_spans(f1, ts, d1);
    //let (result, _) = replace_values_at_spans(f2, rs, d2);

    std::fs::write("test.mdl", f1);
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        parse_optimize_model();
    }
}