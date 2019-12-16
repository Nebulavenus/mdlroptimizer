extern crate took;
extern crate clap;
extern crate pest;
extern crate regex;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate took_macro;

use clap::{App, Arg, ArgMatches, SubCommand};

mod util;
mod parser;
mod model;
mod optimizer;

use parser::parse_file;
use optimizer::optimize_model;
use crate::util::{remove_redundant_lines, replace_values_at_spans};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;

// TODO(nv): rewrite this in future
#[took(description = "done model optimizing in")]
pub fn parse_optimize_model(path: &Path) {
    let file_name = path.file_stem().unwrap();
    let mut file = File::open(path).expect("cannot find file");
    let buf_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut raw_string = String::with_capacity(buf_size);
    file.read_to_string(&mut raw_string).expect("good");
    //let raw_file = fs::read_to_string(path).expect("cannot read file");

    let (file, model) = parse_file(&raw_string);
    let (redundant_lines, _, _) = optimize_model(model);
    let file1 = remove_redundant_lines(file.clone(), redundant_lines);


    let (file2, model1) = parse_file(&file1);
    let (_, translation_spans, _) = optimize_model(model1);
    let file3 = replace_values_at_spans(file2, translation_spans);

    let (file4, model2) = parse_file(&file3);
    let (_, _, rotation_spans) = optimize_model(model2);
    let file5 = replace_values_at_spans(file4, rotation_spans);
    //let (result, _) = replace_values_at_spans(f2, rs, d2);

    let new_file_name =
        String::from(file_name.to_str().unwrap()) + String::from("_optimized.mdl").as_ref();

    std::fs::write(new_file_name, file5);
}

fn main() {
    let matches = App::new("   Mdlroptimizer")
        .version("0.1.0")
        .about("   Tool for optimizing mdl models.")
        .author("   Nebula Venus (Github)")
        /*
        .arg(Arg::with_name("dir")
            .help("Process all files in directory")
            .takes_value(true)
            .short("p")
            .short("parse-all"))
        */
        .arg(Arg::with_name("file")
            .help("Optimize mdl file")
            .takes_value(true)
            .long("optimize"))
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        parse_optimize_model(file.as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        parse_optimize_model("././testfiles/ChaosWarrior_opt1.mdl".as_ref());
    }
}