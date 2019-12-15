use crate::parser::{Rule, parse_field};

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
    pub rotation: Vec<Frame>,
}

#[derive(Default, Debug)]
pub struct Frame {
    pub num: u32,
    pub value: [f32; 3],
}