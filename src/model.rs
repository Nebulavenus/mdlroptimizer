use crate::parser::{Rule, parse_field, parse_bone_field, parse_bone_field_keys};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub sequences: Vec<Anim>,
    pub bones: Vec<Bone>,
}

impl Model {
    pub fn parse(&mut self, inner_model: pest::iterators::Pairs<'_, Rule>) {
        inner_model
            .map(|pair| {
                match pair.as_rule() {
                    Rule::field_name => {
                        dbg!(pair.as_str());
                        self.name = String::from(pair.as_str());
                    },
                    _ => (),
                }
            })
            .for_each(drop);
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Anim {
    pub name: String,
    pub interval: [u32; 2],
}

impl Anim {
    pub fn parse(&mut self, inner_sequence: pest::iterators::Pairs<'_, Rule>) {
        let mut anim_iter = inner_sequence
            .map(|pair| {
                match pair.as_rule() {
                    Rule::section_name => {
                        self.name = String::from(pair.as_str());
                    },
                    Rule::field => {
                        // Problematic area/Check for field name in future
                        let (_, values) = parse_field(pair.into_inner().clone());
                        let array: [u32; 2] = [values[0] as u32, values[1] as u32];
                        self.interval = array;
                    },
                    _ => (),
                }

            });
        // Parse anim name
        anim_iter.next().unwrap();
        // Parse interval field
        anim_iter.next().unwrap();
    }
}

#[derive(Default, Debug)]
pub struct Bone {
    pub name: String,
    pub translations: Vec<Frame>,
    pub rotations: Vec<Frame>,
}

impl Bone {
    pub fn parse(&mut self, inner_bone: pest::iterators::Pairs<'_, Rule>) {

        inner_bone
            .clone()
            .map(|pair| {
                match pair.as_rule() {
                    Rule::field_name => {
                        self.name = String::from(pair.as_str());
                    },
                    Rule::bone_field => {
                        //println!("{}", pair);
                        let inner_bone_field = pair.into_inner();
                        let (translations, rotations)
                            = parse_bone_field(inner_bone_field.clone());

                        if !translations.is_empty() {
                            self.translations = translations;
                        }
                        if !rotations.is_empty() {
                            self.rotations = rotations;
                        }
                    }
                    _ => (),
                }
            })
            .for_each(drop);
    }
}

#[derive(Default, Debug)]
pub struct Frame {
    pub name: u32,
    pub values: [f32; 3],
}

impl Frame {
    pub fn parse(&mut self, inner_bone_field_keys: pest::iterators::Pairs<'_, Rule>) {
        let (name, values) = parse_bone_field_keys(inner_bone_field_keys);
        self.name = name;
        let array: [f32; 3] = [values[0], values[1], values[2]];
        self.values = array;
    }
}