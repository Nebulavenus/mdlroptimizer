use crate::parser::{Rule, parse_field, parse_bone_field, parse_bone_field_keys};
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

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
    pub translation_span: [usize; 2],
    pub rotation_span: [usize; 2],
    pub translation_frames: Vec<Frame>,
    pub rotation_frames: Vec<Frame>,
    pub translation_spans: Vec<[usize; 2]>,
    pub rotation_spans: Vec<[usize; 2]>,
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
                        let (translations, rotations,
                            translation_span, rotation_span,
                            translation_spans, rotation_spans)
                            = parse_bone_field(inner_bone_field.clone());

                        if !translations.is_empty() {
                            self.translation_spans = translation_spans.iter()
                                .map(|span| [span.start(), span.end()])
                                .collect();
                            self.translation_frames = translations;
                        }
                        if !rotations.is_empty() {
                            self.rotation_spans = rotation_spans.iter()
                                .map(|span| [span.start(), span.end()])
                                .collect();
                            self.rotation_frames = rotations;
                        }
                        if !translation_span.as_str().is_empty() {
                            self.translation_span = [translation_span.start(), translation_span.end()];
                        }
                        if !rotation_span.as_str().is_empty() {
                            self.rotation_span = [rotation_span.start(), rotation_span.end()];
                        }
                    }
                    _ => (),
                }
            })
            .for_each(drop);
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Frame {
    pub name: u32,
    pub values: [f32; 3],
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.values[0] == other.values[0] &&
        self.values[1] == other.values[1] &&
        self.values[2] == other.values[2]
    }
}

impl Frame {
    pub fn parse(&mut self, inner_bone_field_keys: pest::iterators::Pairs<'_, Rule>) {
        let (name, values) = parse_bone_field_keys(inner_bone_field_keys);
        self.name = name;
        let array: [f32; 3] = [values[0], values[1], values[2]];
        self.values = array;
    }
}