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
        let input = r#"Model "Name" {
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
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_sequences_section() {
        let input = r#"Sequences 4 {
            Anim "Stand 1 - Portrant" {
                Interval { 416, 3416 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "Stand 2" {
                Interval { 3750, 5875 },
                Rarity 3,
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "Walk" {
                Interval { 6250, 7500 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "StandReady" {
                Interval { 7916, 9166 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_global_sequences_section() {
        let input = r#"GlobalSequences 2 {
            Duration 3000,
            Duration 200,
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_bone_section() {
        let input = r#"Bone "Root" {
            ObjectId 0,
            Translation 7 {
                Linear,
                41: { 0, 0, 0 },
                416: { 0, 0, -1.79688 },
                1583: { 0.0306396, 0, -3.0625 },
                1916: { 0.0334473, 0, -3.125 },
                3416: { 0, 0, -1.79688 },
                5875: { 0, 0, -1.79688 },
                6250: { 0, 0, -7 },
            }
            Rotation 5 {
                Linear,
                41: { 0, 0, 0, 1 },
                416: { 0, 0, 0.291016, 0.953125 },
                5875: { 0, 0, 0.291016, 0.953125 },
                6250: { 0, 0, 0, 1 },
                7500: { 0, 0, 0, 1 },
            }
            GeosetId Multiple,
            GeosetAnimId None,
        }"#;

        parse_dbg(input);
    }
}