use crate::parser::{Rule, parse_field, parse_bone_field, parse_bone_field_keys};

#[derive(Default, Debug)]
pub struct Model {
    pub name: String,
    pub sequences: Vec<Anim>,
    pub gl_sequences: Vec<GlAnim>,
    pub bones: Vec<Bone>,
    pub helpers: Vec<Bone>,
}

impl Model {
    pub fn parse(&mut self, inner_model: pest::iterators::Pairs<'_, Rule>) {
        inner_model
            .map(|pair| {
                match pair.as_rule() {
                    Rule::section_name => {
                        self.name = String::from(pair.as_str());
                    },
                    _ => (),
                }
            })
            .for_each(drop);
    }
}
#[derive(Default, Debug, Eq, PartialEq)]
pub struct GlAnim {
    pub name: String,
    pub duration: u32,
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
    pub translation_frames: Vec<Frame>,
    pub translation_spans: Vec<[usize; 2]>,
    pub rotation_span: [usize; 2],
    pub rotation_frames: Vec<Frame>,
    pub rotation_spans: Vec<[usize; 2]>,
    pub scaling_span: [usize; 2],
    pub scaling_frames: Vec<Frame>,
    pub scaling_spans: Vec<[usize; 2]>,
}

impl Bone {
    pub fn parse(&mut self, inner_bone: pest::iterators::Pairs<'_, Rule>) {

        inner_bone
            .clone()
            .map(|pair| {
                match pair.as_rule() {
                    Rule::section_name => {
                        self.name = String::from(pair.as_str());
                    },
                    Rule::bone_field => {
                        let inner_bone_field = pair.into_inner();
                        let (translation_frames, rotation_frames, scaling_frames,
                            translation_span, rotation_span, scaling_span,
                            translation_spans, rotation_spans, scaling_spans)
                            = parse_bone_field(inner_bone_field.clone());

                        if !translation_frames.is_empty() {
                            self.translation_spans = translation_spans;
                            self.translation_frames = translation_frames;
                        }
                        if !rotation_frames.is_empty() {
                            self.rotation_spans = rotation_spans;
                            self.rotation_frames = rotation_frames;
                        }
                        if !scaling_frames.is_empty() {
                            self.scaling_spans = scaling_spans;
                            self.scaling_frames = scaling_frames;
                        }
                        if translation_span[0] != 0 && translation_span[1] != 0 {
                            self.translation_span = translation_span;
                        }
                        if rotation_span[0] != 0 && rotation_span[1] != 0 {
                            self.rotation_span = rotation_span;
                        }
                        if scaling_span[0] != 0 && scaling_span[1] != 0 {
                            self.scaling_span = scaling_span;
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
    pub values: [Option<f32>; 4],
    pub hermite: bool,
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl Frame {
    pub fn parse(&mut self, inner_bone_field_keys: pest::iterators::Pairs<'_, Rule>) {
        let (name, values, is_hermite) = parse_bone_field_keys(inner_bone_field_keys);
        self.name = name;
        let mut array: [Option<f32>; 4] = [None; 4];
        for (idx, value) in values.iter().enumerate() {
            array[idx] = Some(*value);
        }
        self.values = array;
        self.hermite = is_hermite;
    }
}