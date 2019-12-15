use crate::parser;
use crate::parser::Rule;

#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub sequences: Vec<Anim>,
    pub bones: Vec<Bone>,
}

impl Model {
    pub fn parse(&mut self, inner_model: pest::iterators::Pairs<'_, parser::Rule>) {
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
    pub fn parse(&mut self, inner_sequence: pest::iterators::Pairs<'_, parser::Rule>) {
        inner_sequence
            .map(|pair| {
                match pair.as_rule() {
                    Rule::section_name => {
                        self.name = String::from(pair.as_str());
                    },
                    Rule::field => {
                        let interval = pair.into_inner()
                            .clone()
                            .next().unwrap();
                        //dbg!(interval);
                        //anim.interval = String::
                    },
                    _ => (),
                }

            })
            .for_each(drop);
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