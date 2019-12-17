use pest::Span;
use crate::model::{Model, Frame, Bone};
use std::collections::{HashMap, HashSet};
use std::ops::Range;

pub fn bone_section_spans(model: Model) -> (Vec<(String, [usize; 2])>, Vec<(String, [usize; 2])>) {
    let mut translation_section_spans = Vec::new();
    let mut rotation_section_spans= Vec::new();
    for bone in model.bones.iter() {
        translation_section_spans.push((bone.name.clone(), bone.translation_span));
        rotation_section_spans.push((bone.name.clone(), bone.rotation_span));
    }
    (translation_section_spans, rotation_section_spans)
}

pub fn optimize_model(model: Model) -> (Vec<[usize; 2]>, Vec<(String, u32)>, Vec<(String, u32)>) {
    // Duplicates frames
    let mut delete_spans = Vec::<[usize; 2]>::new();

    // Span and new value
    let mut translation_section_values: Vec<(String, u32)> = Vec::new();
    let mut rotations_section_values: Vec<(String, u32)> = Vec::new();

    // Start and end frames for animation
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<Range<u32>>::new();

    for anim in model.sequences {
        special_frames.push(anim.interval[0]);
        special_frames.push(anim.interval[1]);
        anim_frame_ranges.push((anim.interval[0]..anim.interval[1]));
    }
    //println!("{:?}", &special_frames);

    for (idx, bone) in model.bones.iter().enumerate() {
        let mut unique_frames = Vec::<Frame>::new();
        let mut deleted_spans_count = 0usize;
        for (idx, frame) in bone.translation_frames.iter().enumerate() {
            // Try to insert unique frame, if not mark lines to delete.
            match unique_frames.pop() {
                None => {
                    let key = frame.name;
                    let frame_in_range = anim_frame_ranges
                        .iter()
                        .any(|range| range.contains(&key));
                    if frame_in_range {
                        unique_frames.push(*frame);
                    } else {
                        deleted_spans_count += 1;
                        delete_spans.push(bone.translation_spans[idx]);
                    }
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frames.push(*frame);
                    } else {
                        if !special_frames.contains(&frame.name) {
                            //dbg!(&frame.name);
                            delete_spans.push(bone.translation_spans[idx]);
                        }
                    }
                }
            }
        }
        //if !unique_frames.is_empty() {
        if !bone.translation_spans.is_empty() {
            translation_section_values.push((
                bone.name.clone(),
                //(bone.translation_frames.len() - unique_frames.len()) as u32
                //unique_frames.len() as u32
                //deleted_spans_count as u32
                (bone.translation_spans.len() - deleted_spans_count) as u32
            ));
        }
        //}

        unique_frames.clear();
        deleted_spans_count = 0;
        for (idx, frame) in bone.rotation_frames.iter().enumerate() {
            // Try to insert unique frame, if not mark lines to delete.
            match unique_frames.pop() {
                None => {
                    let key = frame.name;
                    let frame_in_range = anim_frame_ranges
                        .iter()
                        .any(|range| range.contains(&key));
                    if frame_in_range {
                        unique_frames.push(*frame);
                    } else {
                        delete_spans.push(bone.rotation_spans[idx]);
                        deleted_spans_count += 1;
                    }
                },
                Some(vec_frame) => {
                    if frame.values != vec_frame.values {
                        unique_frames.push(*frame);
                    } else {
                        if !special_frames.contains(&frame.name) {
                            delete_spans.push(bone.rotation_spans[idx]);
                        }
                    }
                }
            }
        }
        //if !unique_frames.is_empty() {
        if !bone.rotation_spans.is_empty() {
            rotations_section_values.push((
                bone.name.clone(),
                //(bone.rotation_frames.len() - unique_frames.len()) as u32
                //unique_frames.len() as u32
                (bone.rotation_spans.len() - deleted_spans_count) as u32
            ));
        }
        //}

        //unique_frames.insert(idx, unique_frame);
    }
    dbg!(&translation_section_values);

    (delete_spans, translation_section_values, rotations_section_values)
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
        println!("{:?}", redundant_lines.0);
        println!("{:?}", redundant_lines.1);
        println!("{:?}", redundant_lines.2);
    }
}
