use pest::Span;
use crate::model::{Model, Frame, Bone};
use std::collections::{HashMap, HashSet};

pub fn optimize_model(model: Model)
    -> (Vec<[usize; 2]>, Vec<([usize; 2], u32)>, Vec<([usize; 2], u32)>) {
    // Duplicates frames
    let mut delete_spans = Vec::<[usize; 2]>::new();

    // Span and new value
    let mut translation_section_spans: Vec<([usize; 2], u32)> = Vec::new();
    let mut rotations_section_spans: Vec<([usize; 2], u32)> = Vec::new();

    // Start and end frames for animation
    let mut special_frames = Vec::<u32>::new();

    for anim in model.sequences {
        special_frames.push(anim.interval[0]);
        special_frames.push(anim.interval[1]);
    }

    for (idx, bone) in model.bones.iter().enumerate() {
        let mut unique_frame = Vec::<Frame>::new();
        for (idx, frame) in bone.translation_frames.iter().enumerate() {
            // Try to insert unique frame, if not mark lines to delete.
            match unique_frame.pop() {
                None => {
                    unique_frame.push(*frame);
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frame.push(*frame);
                    } else {
                        if !special_frames.contains(&frame.name) {
                            delete_spans.push(bone.translation_spans[idx]);
                        }
                    }
                }
            }
        }
        if !unique_frame.is_empty() {
            translation_section_spans.push((
                bone.translation_span,
                (bone.translation_spans.len() - unique_frame.len()) as u32
            ));
        }

        unique_frame.clear();
        for (idx, frame) in bone.rotation_frames.iter().enumerate() {
            // Try to insert unique frame, if not mark lines to delete.
            match unique_frame.pop() {
                None => {
                    unique_frame.push(*frame);
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frame.push(*frame);
                    } else {
                        if !special_frames.contains(&frame.name) {
                            delete_spans.push(bone.rotation_spans[idx]);
                        }
                    }
                }
            }
        }
        if !unique_frame.is_empty() {
            rotations_section_spans.push((
                bone.rotation_span,
                (bone.rotation_spans.len() - unique_frame.len()) as u32
            ));
        }

        //unique_frames.insert(idx, unique_frame);
    }

    (delete_spans, translation_section_spans, rotations_section_spans)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;
    use std::ops::RangeBounds;
    use std::rc::Rc;
    use crate::util::remove_redundant_lines;

    #[test]
    fn optimize_api_model() {
        let (file, model) = parse_file("././testfiles/ChaosWarrior_opt1.mdl");
        let redundant_lines = optimize_model(model);
        println!("{:?}", redundant_lines.0);
        println!("{:?}", redundant_lines.1);
        println!("{:?}", redundant_lines.2);
    }
}
