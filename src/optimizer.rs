use pest::Span;
use crate::model::{Model, Frame, Bone};
use std::collections::{HashMap, HashSet};
use std::ops::{Range, RangeInclusive};

pub fn bone_section_spans_count(model: Model) -> (Vec<([usize; 2], u32)>, Vec<([usize; 2], u32)>) {
    let mut translation_section_spans = Vec::new();
    let mut rotation_section_spans= Vec::new();
    for bone in model.bones.iter() {
        if !bone.translation_frames.is_empty() {
            translation_section_spans.push((bone.translation_span, bone.translation_frames.len() as u32));
        }
        if !bone.rotation_frames.is_empty() {
            rotation_section_spans.push((bone.rotation_span, bone.rotation_frames.len() as u32));
        }
    }
    (translation_section_spans, rotation_section_spans)
}

pub fn optimize_model(model: Model) -> Vec<[usize; 2]> {
    // Duplicates frames
    let mut delete_spans = Vec::<[usize; 2]>::new();

    // Start and end frames for animation
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<RangeInclusive<u32>>::new();

    for anim in model.sequences {
        special_frames.push(anim.interval[0]);
        special_frames.push(anim.interval[1]);
        anim_frame_ranges.push((anim.interval[0]..=anim.interval[1]));
    }
    //println!("{:?}", &special_frames);
    println!("{:?}", &anim_frame_ranges);

    for (idx, bone) in model.bones.iter().enumerate() {
        let mut unique_frame = Vec::<Frame>::new();
        let mut in_range_frames = Vec::<(usize, Frame)>::new();
        for (idx, frame) in bone.translation_frames.iter().enumerate() {
            let key = frame.name;
            let frame_in_range = anim_frame_ranges
                .iter()
                .any(|range| range.contains(&key));
            if frame_in_range {
                in_range_frames.push((idx, *frame));
            } else {
                delete_spans.push(bone.translation_spans[idx]);
            }
        }
        /*
        for (idx, frame) in in_range_frames.clone() {
            match unique_frame.pop() {
                None => {
                    unique_frame.push(frame);
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frame.push(frame);
                    } else {
                        //dbg!(&frame.name);
                        delete_spans.push(bone.translation_spans[idx]);
                    }
                }
            }
        }
        */

        unique_frame.clear();
        in_range_frames.clear();
        for (idx, frame) in bone.rotation_frames.iter().enumerate() {
            let key = frame.name;
            let frame_in_range = anim_frame_ranges
                .iter()
                .any(|range| range.contains(&key));
            if frame_in_range {
                in_range_frames.push((idx, *frame));
            } else {
                delete_spans.push(bone.rotation_spans[idx]);
            }
        }
        /*
        for (idx, frame) in in_range_frames {
            match unique_frame.pop() {
                None => {
                    unique_frame.push(frame);
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frame.push(frame);
                    } else {
                        if !special_frames.contains(&frame.name) {
                            //dbg!(&frame.name);
                            delete_spans.push(bone.rotation_spans[idx]);
                        }
                    }
                }
            }
        }
        */
    }

    delete_spans
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;
    use std::fs;

    #[test]
    fn optimize_api_model() {
        let file = fs::read_to_string("././testfiles/ChaosWarrior_opt1.mdl")
            .expect("cannot find file");
        let model = parse_file(&file);
        let redundant_lines = optimize_model(model);
        println!("{:?}", redundant_lines);
    }
}
