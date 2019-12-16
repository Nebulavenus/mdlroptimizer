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
use std::fs;

pub fn parse_optimize_model(path: &str) {
    let raw_file = fs::read_to_string(path).expect("cannot read file");
    let (file, model) = parse_file(&raw_file);
    let (redundant_lines, _, _) = optimize_model(model);
    let file1 = remove_redundant_lines(file.clone(), redundant_lines);


    let (file2, model1) = parse_file(&file1);
    let (_, translation_spans, _) = optimize_model(model1);
    let file3 = replace_values_at_spans(file2, translation_spans);
    //let (result, _) = replace_values_at_spans(f2, rs, d2);

    std::fs::write("test1.mdl", file);
    std::fs::write("test.mdl", file3);
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
        parse_optimize_model("././testfiles/ChaosWarrior_opt1.mdl");
    }
}