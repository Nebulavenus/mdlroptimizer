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
    for helper in model.helpers.iter() {
        if !helper.translation_frames.is_empty() {
            translation_section_spans.push((helper.translation_span, helper.translation_frames.len() as u32));
        }
        if !helper.rotation_frames.is_empty() {
            rotation_section_spans.push((helper.rotation_span, helper.rotation_frames.len() as u32));
        }
    }
    (translation_section_spans, rotation_section_spans)
}

pub fn optimize_model(model: Model) -> Vec<[usize; 2]> {
    // Duplicates frames
    let mut delete_spans = Vec::<[usize; 2]>::new();

    // Start and end frames for animation
    let mut gl_frames = Vec::<u32>::new();
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<RangeInclusive<u32>>::new();

    for anim in model.sequences {
        special_frames.push(anim.interval[0]);
        special_frames.push(anim.interval[1]);
        anim_frame_ranges.push((anim.interval[0]..=anim.interval[1]));
    }

    for gl_anim in model.gl_sequences {
        gl_frames.push(gl_anim.duration);
    }

    // Inside bones
    for (idx, bone) in model.bones.iter().enumerate() {

        // Range translation frames
        {
            let mut in_range_translation_frames = Vec::<(usize, Frame)>::new();
            for (idx, frame) in bone.translation_frames.iter().enumerate() {
                let key = frame.name;
                let frame_in_range = anim_frame_ranges
                    .iter()
                    .any(|range| range.contains(&key));
                if frame_in_range {
                    in_range_translation_frames.push((idx, *frame));
                } else {
                    if !gl_frames.contains(&key) {
                        delete_spans.push(bone.translation_spans[idx]);
                    }
                }
            }
            let mut irtf = in_range_translation_frames
                .iter()
                .peekable();
            while let Some((_, curr_frame)) = irtf.next() {
                if let Some((idx, next_frame)) = irtf.peek() {
                    if curr_frame.values == next_frame.values {
                        if !special_frames.contains(&next_frame.name) {
                            delete_spans.push(bone.translation_spans[*idx]);
                        }
                    }
                }
            }
        }

        // Range rotation frames
        {
            let mut in_range_rotation_frames = Vec::<(usize, Frame)>::new();
            for (idx, frame) in bone.rotation_frames.iter().enumerate() {
                let key = frame.name;
                let frame_in_range = anim_frame_ranges
                    .iter()
                    .any(|range| range.contains(&key));
                if frame_in_range {
                    in_range_rotation_frames.push((idx, *frame));
                } else {
                    if !gl_frames.contains(&key) {
                        delete_spans.push(bone.rotation_spans[idx]);
                    }
                }
            }
            let mut irrf = in_range_rotation_frames
                .iter()
                .peekable();
            while let Some((_, curr_frame)) = irrf.next() {
                if let Some((idx, next_frame)) = irrf.peek() {
                    if curr_frame.values == next_frame.values {
                        if !special_frames.contains(&next_frame.name) {
                            delete_spans.push(bone.rotation_spans[*idx]);
                        }
                    }
                }
            }
        }
    }
    // Inside helpers
    for (idx, helper) in model.helpers.iter().enumerate() {

        // Range translation frames
        {
            let mut in_range_translation_frames = Vec::<(usize, Frame)>::new();
            for (idx, frame) in helper.translation_frames.iter().enumerate() {
                let key = frame.name;
                let frame_in_range = anim_frame_ranges
                    .iter()
                    .any(|range| range.contains(&key));
                if frame_in_range {
                    in_range_translation_frames.push((idx, *frame));
                } else {
                    if !gl_frames.contains(&key) {
                        delete_spans.push(helper.translation_spans[idx]);
                    }
                }
            }
            let mut irtf = in_range_translation_frames
                .iter()
                .peekable();
            while let Some((_, curr_frame)) = irtf.next() {
                if let Some((idx, next_frame)) = irtf.peek() {
                    if curr_frame.values == next_frame.values {
                        if !special_frames.contains(&next_frame.name) {
                            delete_spans.push(helper.translation_spans[*idx]);
                        }
                    }
                }
            }
        }

        // Range rotation frames
        {
            let mut in_range_rotation_frames = Vec::<(usize, Frame)>::new();
            for (idx, frame) in helper.rotation_frames.iter().enumerate() {
                let key = frame.name;
                let frame_in_range = anim_frame_ranges
                    .iter()
                    .any(|range| range.contains(&key));
                if frame_in_range {
                    in_range_rotation_frames.push((idx, *frame));
                } else {
                    if !gl_frames.contains(&key) {
                        delete_spans.push(helper.rotation_spans[idx]);
                    }
                }
            }
            let mut irrf = in_range_rotation_frames
                .iter()
                .peekable();
            while let Some((_, curr_frame)) = irrf.next() {
                if let Some((idx, next_frame)) = irrf.peek() {
                    if curr_frame.values == next_frame.values {
                        if !special_frames.contains(&next_frame.name) {
                            delete_spans.push(helper.rotation_spans[*idx]);
                        }
                    }
                }
            }
        }
    }

    delete_spans.sort();
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
