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
use crate::optimizer::bone_section_spans_count;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[took(description = "Optimizing model...")]
pub fn parse_optimize_model(path: &Path) {
    // Load mdl file at specific path
    let file_name = path.file_stem().unwrap();
    let mut file = File::open(path).expect("cannot find file");
    let buf_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
    let mut raw_string = String::with_capacity(buf_size);
    file.read_to_string(&mut raw_string).expect("good");

    // First mark and delete redundant lines
    let model= parse_file(&raw_string);
    let redundant_lines = optimize_model(model);
    println!("{} redundant lines found.", &redundant_lines.len());
    let processed_string
        = remove_redundant_lines(raw_string, redundant_lines);

    // Replace old values in all bones translation sections
    let model1 = parse_file(&processed_string);
    let (translation_spans, rotation_spans) = bone_section_spans_count(model1);
    let processed_string1
        = replace_values_at_spans(processed_string, translation_spans);
    let final_string
        = replace_values_at_spans(processed_string1, rotation_spans);


    // Output result
    let new_file_name =
        String::from(file_name.to_str().unwrap()) + String::from("_optimized.mdl").as_ref();

    std::fs::write(new_file_name, final_string);
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
            .short("f")
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
        //parse_optimize_model("././testfiles/ChaosWarrior_unopt.mdl".as_ref());
        parse_optimize_model("././testfiles/DruidCat.mdl".as_ref());
    }
}