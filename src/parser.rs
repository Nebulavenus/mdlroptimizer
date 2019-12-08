use pest::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

pub fn parse_dbg(input: &str) {

    let pairs = MDLParser::parse(Rule::mdl, input)
        .expect("unsuccessful parse")
        .next().unwrap();

    dbg!(&pairs);
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

        parse_dbg(input);
    }

    #[test]
    fn parse_model_section() {
        let input = "Model \"Name\" {
            NumGeosets 1,
            NumGeosetAnims 2,
            NumLights 1,
            NumHelpers 1,
            NumBones 5,
            NumAttachments 3,
            NumParticleEmitters 1,
            NumParticleEmitters2 1,
            NumRibbonEmitters 1,
            NumEvents 2,
            BlendTime 150,
            MinimumExtent { -27.125, -23.125, 0.225586 },
            MaximumExtent { 22, 24.25, 98.5 },
            BoundsRadius 34.4232,
        }";

        parse_dbg(input);
    }
}