use pest::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

pub fn parse(input: &str) {

    let pairs = MDLParser::parse(Rule::mdl, input)
        .expect("unsuccessful parse")
        .next().unwrap();

    dbg!(&pairs);

    let mut nodes = vec![];

    let mut current_section_name = "";

    for p in pairs.into_inner() {
        match p.as_rule() {
            Rule::section => {
                let mut inner_rules = p.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
                dbg!(&current_section_name);
                nodes.push(current_section_name);
            },
            Rule::EOI => (),
            _ => (),
        }
    }
    dbg!(&current_section_name);

    println!("{:#?}", nodes);
}

pub fn parse_file(path: String) {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");

    let file = MDLParser::parse(Rule::mdl, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();

    let mut result: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_section() {
        let input = "Version {
            FormatVersion 800,
            Simple 11.24123,
        }";

        parse(input);
    }
}