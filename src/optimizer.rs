use crate::model::{Model, Frame};
use std::ops::RangeInclusive;
use itertools::multipeek;
use std::str::FromStr;

pub fn bone_section_spans_count(model: Model) -> (Vec<([usize; 2], u32)>, Vec<[usize; 2]>) {
    let mut result_sections = Vec::new();
    let mut result_interps = Vec::new();

    let mut translation_section_spans = Vec::new();
    let mut translation_interp_spans = Vec::new();

    let mut rotation_section_spans= Vec::new();
    let mut rotation_interp_spans = Vec::new();

    let mut scaling_section_spans = Vec::new();
    let mut scaling_interp_spans = Vec::new();

    for bone in model.bones.iter() {
        if !bone.translation_frames.is_empty() {
            translation_section_spans.push((bone.translation_span, bone.translation_frames.len() as u32));
            translation_interp_spans.push(bone.translation_interp_span);
        }
        if !bone.rotation_frames.is_empty() {
            rotation_section_spans.push((bone.rotation_span, bone.rotation_frames.len() as u32));
            rotation_interp_spans.push(bone.rotation_interp_span);
        }
        if !bone.scaling_frames.is_empty() {
            scaling_section_spans.push((bone.scaling_span, bone.scaling_frames.len() as u32));
            scaling_interp_spans.push(bone.scaling_interp_span);
        }
    }
    for helper in model.helpers.iter() {
        if !helper.translation_frames.is_empty() {
            translation_section_spans.push((helper.translation_span, helper.translation_frames.len() as u32));
            translation_interp_spans.push(helper.translation_interp_span);
        }
        if !helper.rotation_frames.is_empty() {
            rotation_section_spans.push((helper.rotation_span, helper.rotation_frames.len() as u32));
            rotation_interp_spans.push(helper.rotation_interp_span);
        }
        if !helper.scaling_frames.is_empty() {
            scaling_section_spans.push((helper.scaling_span, helper.scaling_frames.len() as u32));
            scaling_interp_spans.push(helper.scaling_interp_span);
        }
    }
    result_sections.extend(translation_section_spans);
    result_sections.extend(rotation_section_spans);
    result_sections.extend(scaling_section_spans);
    result_sections.sort();

    result_interps.extend(translation_interp_spans);
    result_interps.extend(rotation_interp_spans);
    result_interps.extend(scaling_interp_spans);
    result_interps.sort();

    (result_sections, result_interps)
}

pub fn optimize_model(model: Model, threshold: f64, outside: bool, linearize: bool) -> Vec<[usize; 2]> {
    // Duplicates frames
    let mut delete_spans = Vec::<[usize; 2]>::new();

    // Start and end frames for animation
    let mut gl_frames = Vec::<u32>::new();
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<RangeInclusive<u32>>::new();

    for anim in model.sequences {
        special_frames.push(anim.interval[0]);
        special_frames.push(anim.interval[1]);
        anim_frame_ranges.push(anim.interval[0]..=anim.interval[1]);
    }

    for gl_anim in model.gl_sequences {
        gl_frames.push(gl_anim.duration);
    }

    let mut collect_frames_from = |frames: &Vec<Frame>, key_spans: &Vec<[usize; 2]>, tan_spans: &Vec<[usize; 2]>| {
        let mut in_range_frames = Vec::<(usize, Frame)>::new();
        for (idx, frame) in frames.iter().enumerate() {
            let key = u32::from_str(frame.name.as_ref()).unwrap();
            let frame_in_range = anim_frame_ranges
                .iter()
                .any(|range| range.contains(&key));
            if frame_in_range {
                if !outside {
                    // Forced linearization
                    if linearize {
                        if !tan_spans.is_empty() {
                            // InTan + OutTan
                            delete_spans.push(tan_spans[idx]);
                            delete_spans.push(tan_spans[idx+1]);
                        }
                    }
                    in_range_frames.push((idx, frame.clone()));
                    /*
                    if frame.hermite {
                        delete_spans.push(key_spans[idx]);
                    } else {
                        in_range_frames.push((idx, *frame));
                    }
                    */
                }
            } else {
                if !gl_frames.contains(&key) {
                    // Key + InTan + OutTan
                    delete_spans.push(key_spans[idx]);
                    if !tan_spans.is_empty() {
                        delete_spans.push(tan_spans[idx]);
                        delete_spans.push(tan_spans[idx+1]);
                    }
                }
            }
        }
        let mut skip_by = 0;
        let mut irf = multipeek(in_range_frames.iter());
        'next: while let Some((_, curr_frame)) = irf.next() {
            if skip_by > 0 {
                skip_by -= 1;
                continue;
            }
            let mut potential_frames = Vec::new();
            'peek: while let Some((next_idx, next_frame)) = irf.peek() {
                let mut identical = true;
                // Threshold
                for i in 0..4 {
                    if let Some(curr_val) = curr_frame.values[i] {
                        let next_val = next_frame.values[i].unwrap();
                        let diff = curr_val - next_val;
                        if diff > threshold as f32 || diff < -threshold as f32 {
                            identical = false;
                            break;
                        }
                    }
                }
                if identical {
                    skip_by += 1;
                    let name = u32::from_str(next_frame.name.as_ref()).unwrap();
                    if !special_frames.contains(&name) {
                        potential_frames.push((*next_idx, key_spans[*next_idx]));
                    }
                } else {
                    skip_by += 1;
                    break 'peek;
                }
            }
            if !potential_frames.is_empty() {
                potential_frames.pop().unwrap();
                for (idx, frame_key_span) in potential_frames {
                    // Key + InTan + OutTan
                    delete_spans.push(frame_key_span);
                    delete_spans.push(tan_spans[idx]);
                    delete_spans.push(tan_spans[idx+1]);
                }
            }
        }
    };

    // Inside bones
    for (_idx, bone) in model.bones.iter().enumerate() {

        // Range translation frames
        collect_frames_from(
            bone.translation_frames.as_ref(),
            bone.translation_key_spans.as_ref(),
            bone.translation_tan_spans.as_ref(),
        );

        // Range rotation frames
        collect_frames_from(
            bone.rotation_frames.as_ref(),
            bone.rotation_key_spans.as_ref(),
            bone.rotation_tan_spans.as_ref(),
        );

        // Range scaling frames
        collect_frames_from(
            bone.scaling_frames.as_ref(),
            bone.scaling_key_spans.as_ref(),
            bone.scaling_tan_spans.as_ref(),
        );
    }
    // Inside helpers
    for (_idx, helper) in model.helpers.iter().enumerate() {

        // Range translation frames
        collect_frames_from(
            helper.translation_frames.as_ref(),
            helper.translation_key_spans.as_ref(),
            helper.translation_tan_spans.as_ref(),
        );

        // Range rotation frames
        collect_frames_from(
            helper.rotation_frames.as_ref(),
            helper.rotation_key_spans.as_ref(),
            helper.rotation_tan_spans.as_ref(),
        );

        // Range scaling frames
        collect_frames_from(
            helper.scaling_frames.as_ref(),
            helper.scaling_key_spans.as_ref(),
            helper.scaling_tan_spans.as_ref(),
        );
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
        let (model, parsed) = parse_file(file);
        let redundant_lines = optimize_model(model, 0.0, true, false);
        println!("{:?}", redundant_lines);
    }
}
