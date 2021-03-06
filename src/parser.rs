use pest::Parser;
use std::str::FromStr;
use crate::model::{Model, Anim, Bone, Frame, GlAnim};
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

fn parse_dbg(input: &str) {

    let pairs = MDLParser::parse(Rule::mdl, input)
        .expect("unsuccessful parse")
        .next().unwrap();

    dbg!(&pairs);
}

pub fn parse_field(inner_field: Pairs<'_, Rule>) -> (String, Vec<f32>) {
    let mut field_name = String::new();
    let mut values = Vec::<f32>::new();

    inner_field
        .map(|pair| {
            match pair.as_rule() {
                Rule::data => {
                    let inner_data = pair.into_inner();
                    inner_data
                        .clone()
                        .map(|pair| {
                            //dbg!(&pair);
                            match pair.as_rule() {
                                Rule::field_name => {
                                    field_name = String::from(pair.as_str());
                                },
                                Rule::value => {
                                    let number
                                        = f32::from_str(pair.as_str()).unwrap();
                                    values.push(number);
                                },
                                Rule::complex_value => {
                                    for inner_complex_value in pair.into_inner().clone() {
                                        let number =
                                            f32::from_str(inner_complex_value.as_str()).unwrap();
                                        values.push(number);
                                    }
                                },
                                _ => (),
                            }
                        })
                        .for_each(drop);
                },
                _ => (),
            }
        })
        .for_each(drop);
    //dbg!(&field_name, &values);
    (field_name, values)
}

pub fn parse_bone_field_keys(inner_bone_keys: Pairs<'_, Rule>) -> (String, Vec<f32>) {
    let mut name= String::new();
    let mut values = Vec::<f32>::new();

    inner_bone_keys
        .map(|pair| {
            match pair.as_rule() {
                Rule::number => {
                    name = pair.as_str().to_string();
                },
                Rule::field_name => {
                    name = pair.as_str().to_string();
                }
                Rule::complex_value => {
                    for inner_complex_value in pair.into_inner().clone() {
                        let number =
                            f32::from_str(inner_complex_value.as_str()).unwrap();
                        values.push(number);
                    }
                },
                _ => (),
            }
        })
        .for_each(drop);

    (name, values)
}

pub fn parse_bone_field(inner_bone_field: Pairs<'_, Rule>)
                        -> (Vec<Frame>, Vec<Frame>, Vec<Frame>,
                            [usize; 2], [usize; 2], [usize; 2],
                            [usize; 2], [usize; 2], [usize; 2],
                            Vec<[usize; 2]>, Vec<[usize; 2]>, Vec<[usize; 2]>,
                            Vec<[usize; 2]>, Vec<[usize; 2]>, Vec<[usize; 2]>) {

    let mut translation_frames = Vec::<Frame>::new();
    let mut translation_section_span = [0usize; 2];
    let mut translation_interp_span = [0usize; 2];
    let mut translation_key_spans = Vec::<[usize; 2]>::new();
    let mut translation_tan_spans = Vec::<[usize; 2]>::new();

    let mut rotation_frames = Vec::<Frame>::new();
    let mut rotation_section_span = [0usize; 2];
    let mut rotation_interp_span = [0usize; 2];
    let mut rotation_key_spans = Vec::<[usize; 2]>::new();
    let mut rotation_tan_spans = Vec::<[usize; 2]>::new();

    let mut scaling_frames = Vec::<Frame>::new();
    let mut scaling_section_span = [0usize; 2];
    let mut scaling_interp_span = [0usize; 2];
    let mut scaling_key_spans = Vec::<[usize; 2]>::new();
    let mut scaling_tan_spans = Vec::<[usize; 2]>::new();

    let collect_data_in =
        |inner_section: Pairs<Rule>, section_span: &mut [usize; 2], interp_span: &mut [usize; 2],
         frames: &mut Vec<Frame>, key_spans: &mut Vec<[usize; 2]>, tan_spans: &mut Vec<[usize; 2]>| {

            let mut section_count = 0;
            inner_section
                .clone()
                .map(|pair| {
                    match pair.as_rule() {
                        Rule::number => {
                            let span = pair.clone().as_span();
                            *section_span = [span.start(), span.end()];
                            section_count = i32::from_str(pair.as_str()).unwrap();
                        },
                        Rule::interp_type => {
                            let span = pair.clone().as_span();
                            *interp_span = [span.start(), span.end()];
                        },
                        Rule::keys_field => {
                            let span = pair.clone().as_span();
                            key_spans.push([span.start(), span.end()]);

                            let mut frame = Frame::default();
                            let inner_key_field = pair.into_inner();
                            frame.parse_key_field(inner_key_field.clone());
                            frames.push(frame);
                        },
                        Rule::tans_field => {
                            let span = pair.clone().as_span();
                            tan_spans.push([span.start(), span.end()]);

                            if let Some(last_frame) = frames.last_mut() {
                                let inner_tan_field = pair.into_inner();
                                last_frame.parse_tan_field(inner_tan_field.clone());
                            }
                        }
                        _ => (),
                    }
                })
                .for_each(drop);
    };

    inner_bone_field
        .map(|pair| {
            match pair.as_rule() {
                Rule::translation => {
                    let inner_translation = pair.into_inner();
                    collect_data_in(
                        inner_translation,
                        &mut translation_section_span,
                        &mut translation_interp_span,
                        &mut translation_frames,
                        &mut translation_key_spans,
                        &mut translation_tan_spans,
                    );
                },
                Rule::rotation => {
                    let inner_rotation = pair.into_inner();
                    collect_data_in(
                        inner_rotation,
                        &mut rotation_section_span,
                        &mut rotation_interp_span,
                        &mut rotation_frames,
                        &mut rotation_key_spans,
                        &mut rotation_tan_spans,
                    );
                },
                Rule::scaling => {
                    let inner_scaling = pair.into_inner();
                    collect_data_in(
                        inner_scaling,
                        &mut scaling_section_span,
                        &mut scaling_interp_span,
                        &mut scaling_frames,
                        &mut scaling_key_spans,
                        &mut scaling_tan_spans,
                    );
                }
                _ => (),
            }
        })
        .for_each(drop);

    (translation_frames, rotation_frames, scaling_frames,
     translation_section_span, rotation_section_span, scaling_section_span,
     translation_interp_span, rotation_interp_span, scaling_interp_span,
     translation_key_spans, rotation_key_spans, scaling_key_spans,
     translation_tan_spans, rotation_tan_spans, scaling_tan_spans)
}

pub fn parse_file(input: String) -> (Model, String) {
    use crate::util::remove_comments;
    use crate::util::remove_tabs_newlines;
    let removed_comments = remove_comments(&input);
    let removed_tabsnewlins = remove_tabs_newlines(&removed_comments);
    let result = removed_tabsnewlins;
    //println!("{:?}", &result);

    let pairs = MDLParser::parse(Rule::mdl, &result)
        .expect("unsuccessful parse")
        .next().unwrap().into_inner();

    let mut model = Model::default();

    for pair in pairs.clone() {
        match pair.as_rule() {
            Rule::section => {
                let inner_section = pair.into_inner();
                inner_section
                    .clone()
                    .map(|pair| {
                        match pair.as_rule() {
                            Rule::model => {
                                let inner_model = pair.into_inner();
                                model.parse(inner_model.clone());
                            },
                            Rule::global_sequences => {
                                let inner_gl_sequences = pair.into_inner();
                                inner_gl_sequences
                                    .clone()
                                    .map(|pair| {
                                        match pair.as_rule() {
                                            Rule::field => {
                                                let mut gl_anim = GlAnim::default();
                                                let (name, values)
                                                    = parse_field(pair.into_inner().clone());
                                                gl_anim.name = name;
                                                gl_anim.duration = values[0] as u32;
                                                model.gl_sequences.push(gl_anim);
                                            },
                                            _ => (),
                                        }
                                    })
                                    .for_each(drop);
                            }
                            Rule::sequences => {
                                let inner_sequences = pair.into_inner();
                                inner_sequences
                                    .clone()
                                    .map(|pair| {
                                        match pair.as_rule() {
                                            Rule::sequence => {
                                                let mut anim = Anim::default();
                                                let inner_sequence = pair.into_inner();
                                                anim.parse(inner_sequence.clone());
                                                //dbg!(anim);
                                                model.sequences.push(anim);
                                            },
                                            _ => (),
                                        }
                                    })
                                    .for_each(drop);
                            },
                            Rule::bone => {
                                let mut bone = Bone::default();
                                let inner_bone = pair.into_inner();
                                bone.parse(inner_bone.clone());
                                model.bones.push(bone);
                            },
                            Rule::helper => {
                                let mut bone = Bone::default();
                                let inner_bone = pair.into_inner();
                                bone.parse(inner_bone.clone());
                                model.helpers.push(bone);
                            }
                            _ => (),
                        }
                    })
                    .for_each(drop);
            }
            _ => (),
        }
    }

    //println!("{:#?}", model);
    (model, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::fs;

    #[test]
    fn parse_api_file() {
        let file = fs::read_to_string("././testfiles/ChaosWarrior_unopt.mdl")
            .expect("cannot find file");
        let (model, file) = parse_file(file);
    }

    #[test]
    fn parse_full_file() {
        let raw_file = fs::read_to_string("././testfiles/ChaosWarrior_unopt.mdl")
            .expect("not existing file");

        use crate::util::remove_comments;
        let unparsed_file = remove_comments(&raw_file);

        let pairs = MDLParser::parse(Rule::mdl, &unparsed_file)
            .expect("unsuccessful parse")
            .next().unwrap();

        // For visual representation, but if file is too big, this test will terminate
        //println!("{}", pairs.into_inner());
    }

    #[test]
    fn parse_version_section() {
        let input = "Version {
            FormatVersion 800,
            Simple 11.24123,
        }";

        parse_dbg(input);
    }

    #[test]
    fn parse_model_section() {
        let input = r#"Model "Name" {
            NumGeosets 1,
            NumGeosetAnims 2,
            NumLights 1,
            NumHelpers 1,
            NumBones 5,
            NumAttachments 3,
            NumParticleEmitters 1,
            NumParticleEmitters2 1,
            NumRibbonEmitters 1,
            NumEvents 2,
            BlendTime 150,
            MinimumExtent { -27.125, -23.125, 0.225586 },
            MaximumExtent { 22, 24.25, 98.5 },
            BoundsRadius 34.4232,
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_sequences_section() {
        let input = r#"Sequences 4 {
            Anim "Stand 1 - Portrant" {
                Interval { 416, 3416 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "Stand 2" {
                Interval { 3750, 5875 },
                Rarity 3,
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "Walk" {
                Interval { 6250, 7500 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
            Anim "StandReady" {
                Interval { 7916, 9166 },
                MinimumExtent { -35.625, -44.6673, 0.450756 },
                MaximumExtent { 128.058, 44.5145, 161.041 },
                BoundsRadius 99.7335,
            }
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_global_sequences_section() {
        let input = r#"GlobalSequences 2 {
            Duration 3000,
            Duration 200,
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_bone_section() {
        let input = r#"Bone "Root" {
            ObjectId 0,
            Translation 7 {
                Linear,
                41: { 0, 0, 0 },
                416: { 0, 0, -1.79688 },
                1583: { 0.0306396, 0, -3.0625 },
                1916: { 0.0334473, 0, -3.125 },
                3416: { 0, 0, -1.79688 },
                5875: { 0, 0, -1.79688 },
                6250: { 0, 0, -7 },
            }
            Rotation 5 {
                Linear,
                41: { 0, 0, 0, 1 },
                416: { 0, 0, 0.291016, 0.953125 },
                5875: { 0, 0, 0.291016, 0.953125 },
                6250: { 0, 0, 0, 1 },
                7500: { 0, 0, 0, 1 },
            }
            GeosetId Multiple,
            GeosetAnimId None,
        }"#;

        parse_dbg(input);
    }

    #[test]
    fn parse_texture_section() {
        let input = r#"Textures 8 {
            Bitmap {
                Image "Textures\GenericGlowX_Mod2.blp",
            }
            Bitmap {
                Image "Textures\Red_Star3.blp",
            }
            Bitmap {
                Image "Textures\red_Glow3.blp",
            }
            Bitmap {
                Image "ChaosWarrior.blp",
                WrapWidth,
                WrapHeight,
            }
            Bitmap {
                Image "Textures\DemonGate.blp",
                WrapWidth,
                WrapHeight,
            }
            Bitmap {
                Image "",
                ReplaceableId 2,
            }
            Bitmap {
                Image "",
                ReplaceableId 1,
            }
            Bitmap {
                Image "Textures\Footman.blp",
            }
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_material_section() {
        let input = r#"Materials 6 {
            Material {
                Layer {
                    FilterMode None,
                    static TextureID 6,
                    TwoSided,
                }
                Layer {
                    FilterMode Blend,
                    static TextureID 3,
                    TwoSided,
                }
            }
            Material {
                Layer {
                    FilterMode None,
                    static TextureID 6,
                    TwoSided,
                }
                Layer {
                    FilterMode Blend,
                    static TextureID 3,
                    TwoSided,
                }
            }
            Material {
                Layer {
                    FilterMode None,
                    static TextureID 6,
                    TwoSided,
                }
                Layer {
                    FilterMode Blend,
                    static TextureID 3,
                    TwoSided,
                }
            }
            Material {
                Layer {
                    FilterMode Transparent,
                    static TextureID 3,
                    TwoSided,
                }
            }
            Material {
                Layer {
                    FilterMode Transparent,
                    static TextureID 4,
                }
            }
            Material {
                Layer {
                    FilterMode Additive,
                    static TextureID 5,
                    Unshaded,
                }
            }
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_texture_anims_section() {
        let input = r#"TextureAnims 1 {
            TVertexAnim {
                Translation 2 {
                    Linear,
                    GlobalSeqId 0,
                    0: { 0, 0, 0 },
                    3000: { 0, -1, 0 },
                }
            }
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_geoset_section() {
        let input = r#"Geoset {
            Vertices 43 {
                { 9.6875, -5.65625, 137 },
                { 13.6875, -9, 130 },
                { 14.625, -6.8125, 137 },
                { 8.5, -9.125, 132 },
                { 13.6875, -13.1875, 125.5 },
                { 2.5625, -9.625, 131 },
                { -4.59375, -12.1875, 125 },
                { -2.39063, -6.34375, 144 },
                { -10.1875, 0.147461, 139 },
                { -2.96875, -0.160156, 146 },
                { -2.39063, 6.65625, 144 },
                { 9.75, -0.160156, 148 },
                { 12.6875, 6.59375, 143 },
                { 12.6875, -6.28125, 143 },
                { 19.25, -0.160156, 140 },
                { 14.6875, 7.125, 137 },
                { 8.5, 9.375, 132 },
                { 2.5625, 9.9375, 131 },
                { 13.8125, 9.3125, 130 },
                { 13.75, 13.5, 125.5 },
                { -4.59375, 12.4375, 125 },
                { -4.59375, -12.1875, 125 },
                { -10.1875, 0.147461, 139 },
                { -12.875, -0.163086, 117.5 },
                { -4.59375, 12.4375, 125 },
                { 9.3125, -10.375, 121.5 },
                { 13.6875, -13.1875, 125.5 },
                { 13.75, 13.5, 125.5 },
                { 9.4375, 10.6875, 121.5 },
                { 6.4375, 8.0625, 133 },
                { 17, 3.39063, 131 },
                { 4.875, 9.4375, 126 },
                { 17, 0.182617, 124 },
                { 6.4375, -7.875, 133 },
                { 4.875, -9.375, 126 },
                { 17, -3.01563, 131 },
                { 17, -3.01563, 131 },
                { 17, 0.182617, 124 },
                { 17, 3.39063, 131 },
                { 17, 0.182617, 124 },
                { 4.875, -9.375, 126 },
                { 17.875, 0.182617, 113 },
                { 4.875, 9.4375, 126 },
            }
            Normals 43 {
                { 0, 0, 0 },
                { 0.206111, -0.851948, 0.481354 },
                { 0.379033, -0.889609, 0.254815 },
                { 0.115445, -0.924859, 0.362364 },
                { 0.0210518, -0.999778, 0.000310552 },
                { 0.0168612, -0.924129, 0.381709 },
                { -0.410566, -0.910155, 0.0552601 },
                { -0.218529, -0.787981, 0.575614 },
                { -0.933119, -0.00639745, 0.359512 },
                { -0.44812, -0.0134412, 0.893872 },
                { -0.218309, 0.779247, 0.587465 },
                { 0.207318, -0.0178919, 0.97811 },
                { 0.379422, 0.770301, 0.512518 },
                { 0.370203, -0.784325, 0.497779 },
                { 0.795739, -0.0190497, 0.60534 },
                { 0.383742, 0.885326, 0.262564 },
                { 0.111632, 0.924674, 0.364029 },
                { 0.0186976, 0.923422, 0.383332 },
                { 0.191909, 0.854296, 0.483062 },
                { 0.0171849, 0.99985, 0.00217481 },
                { -0.415842, 0.908076, 0.0497275 },
                { -0.410566, -0.910155, 0.0552601 },
                { -0.933119, -0.00639745, 0.359512 },
                { -0.99225, -0.00593451, 0.124117 },
                { -0.415842, 0.908076, 0.0497275 },
                { -0.0304585, -0.83302, -0.552403 },
                { 0.0210518, -0.999778, 0.000310552 },
                { 0.0171849, 0.99985, 0.00217481 },
                { -0.0333061, 0.834117, -0.550581 },
                { 0.416441, 0.905195, 0.0848505 },
                { 0.862275, 0.500068, -0.0800853 },
                { 0.552905, 0.832025, -0.045065 },
                { 0.98664, 0.00480614, -0.162845 },
                { 0.430972, -0.897237, 0.0960659 },
                { 0.56643, -0.823113, -0.0405158 },
                { 0.866623, -0.493288, -0.0750501 },
                { 0.866623, -0.493288, -0.0750501 },
                { 0.98664, 0.00480614, -0.162845 },
                { 0.862275, 0.500068, -0.0800853 },
                { 0.98664, 0.00480614, -0.162845 },
                { 0.56643, -0.823113, -0.0405158 },
                { 0.996821, 0.00782439, 0.0792926 },
                { 0.552905, 0.832025, -0.045065 },
            }
            TVertices 43 {
                { 0.746094, 0.515625 },
                { 0.886719, 0.0183105 },
                { 0.945313, 0.00613403 },
                { 0.894531, 0.0593262 },
                { 0.847656, 0.0229492 },
                { 0.867188, 0.09375 },
                { 0.839844, 0.125 },
                { 0.941406, 0.0874023 },
                { 0.941406, 0.121094 },
                { 0.972656, 0.121094 },
                { 0.941406, 0.0874023 },
                { 0.984375, 0.0810547 },
                { 0.96875, 0.046875 },
                { 0.96875, 0.046875 },
                { 0.988281, 0.012207 },
                { 0.945313, 0.00613403 },
                { 0.894531, 0.0593262 },
                { 0.867188, 0.09375 },
                { 0.886719, 0.0183105 },
                { 0.847656, 0.0229492 },
                { 0.839844, 0.125 },
                { 0.839844, 0.125 },
                { 0.941406, 0.121094 },
                { 0.875, 0.174805 },
                { 0.839844, 0.125 },
                { 0.828125, 0.0216064 },
                { 0.847656, 0.0229492 },
                { 0.847656, 0.0229492 },
                { 0.828125, 0.0216064 },
                { 0.335938, 0.244141 },
                { 0.232422, 0.244141 },
                { 0.294922, 0.1875 },
                { 0.234375, 0.189453 },
                { 0.335938, 0.244141 },
                { 0.294922, 0.1875 },
                { 0.232422, 0.244141 },
                { 0.232422, 0.244141 },
                { 0.234375, 0.189453 },
                { 0.232422, 0.244141 },
                { 0.234375, 0.189453 },
                { 0.294922, 0.1875 },
                { 0.248047, 0.154297 },
                { 0.294922, 0.1875 },
            }
            VertexGroup  {
                0,
                0,
                0,
                0,
                2,
                0,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                2,
                1,
                1,
                0,
                1,
                1,
                1,
                2,
                2,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            }
            Faces 1 117 {
                Triangles {
                    { 1, 2, 3, 4, 1, 3, 4, 3, 5, 6, 4, 5, 6, 5, 7, 6, 7, 8, 9, 8, 7, 8, 9, 10, 11, 10, 9, 11, 12, 10, 5, 3, 7, 3, 13, 7, 3, 2, 13, 2, 14, 13, 13, 14, 11, 13, 11, 7, 7, 11, 9, 14, 12, 11, 14, 15, 12, 16, 17, 10, 12, 16, 10, 15, 16, 12, 15, 18, 16, 18, 19, 16, 16, 19, 17, 19, 20, 17, 17, 20, 10, 10, 20, 8, 21, 22, 23, 22, 24, 23, 25, 26, 21, 27, 28, 24, 29, 30, 31, 31, 30, 32, 33, 34, 35, 34, 32, 35, 36, 37, 38, 39, 40, 41, 39, 41, 42 },
                }
            }
            Groups 3 4 {
                Matrices { 15 },
                Matrices { 8 },
                Matrices { 8, 15 },
            }
            MinimumExtent { -12.875, -13.1875, 113 },
            MaximumExtent { 19.25, 13.5, 148 },
            BoundsRadius 54.4907,
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            Anim {
                MinimumExtent { -12.875, -13.1875, 113 },
                MaximumExtent { 19.25, 13.5, 148 },
                BoundsRadius 54.4907,
            }
            MaterialID 1,
            SelectionGroup 0,
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_attachment_section() {
        let input = r#"Attachment "Origin Ref" {
            ObjectId 20,
            Parent 0,
            DontInherit { Scaling },
            AttachmentID 0,
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_pivot_points_section() {
        let input = r#"PivotPoints 43 {
            { 0, 0.145833, 80.1763 },
            { -0.000135, -10.8541, 78.1761 },
            { 1.80695, -13.6554, 45.9761 },
            { -0.189076, -14.8679, 11.9764 },
            { 0, 10.3458, 77.7763 },
            { 1.8, 12.1458, 45.1763 },
            { -0.204145, 13.5414, 11.3761 },
            { 0, -0.254167, 95.3763 },
            { 9e-006, -0.054222, 114.776 },
            { -0.008317, -22.6535, 116.377 },
            { -1.6937, -25.7375, 107.232 },
            { -3.22436, -30.9072, 93.8533 },
            { 0.002235, 25.1447, 118.176 },
            { -1.93618, 27.345, 107.494 },
            { -3.15427, 30.3449, 93.7752 },
            { 2.7e-005, 0.145832, 122.176 },
            { -1.48809, -32.5442, 75.5463 },
            { -1.48905, -33.2266, 67.2473 },
            { -1.48768, 31.7665, 72.5699 },
            { 0, 0, 0 },
            { 0, 0, 0 },
            { 0, 0, 150 },
            { 2.7e-005, 0.14, 125 },
            { 9e-006, -0.05, 110 },
            { -1.4, 31.7, 72.56 },
            { -1.4, -32.5, 75.5 },
            { 50, -33.2, 65 },
            { -0.2, 13.5, 11.3 },
            { -0.2, -14, 11.9 },
            { -1.4, 31.7, 65 },
            { -1.4, 31.7, 65 },
            { -1.4, -33.2, 65 },
            { 67.876, -31.379, 69.11 },
            { -2.6267, 0.38824, 107.17 },
            { -2.6267, 0.38824, 107.17 },
            { 0, 0, 0 },
            { 0, 0, 0 },
            { 0, 0, 0 },
            { 13.5, 0, 0 },
            { 0, -14, 0 },
            { 0, 0, 0 },
            { 0, 0, 0 },
            { 0, 0, 0 },
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_particle_emitter_section() {
        let input = r#"ParticleEmitter2 "star" {
            ObjectId 30,
            Parent 18,
            Additive,
            static Speed 600,
            static Variation 0,
            static Latitude 0,
            static Gravity 25,
            EmissionRate 3 {
                DontInterp,
                0: 0,
                27000: 100,
                27555: 0,
            }
            static Width 0,
            static Length 0,
            SegmentColor {
                Color { 1, 1, 1 },
                Color { 1, 1, 1 },
                Color { 1, 1, 1 },
            },
            Alpha { 255, 255, 0 },
            ParticleScaling { 30, 30, 15 },
            LifeSpanUVAnim { 0, 0, 1 },
            DecayUVAnim { 0, 0, 1 },
            TailUVAnim { 0, 0, 1 },
            TailDecayUVAnim { 0, 0, 1 },
            Rows 1,
            Columns 1,
            TextureID 1,
            Time 1.73277e-039,
            LifeSpan 0.1,
            TailLength 1.82169e-043,
            LineEmitter,
            Unshaded,
            Head,
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_event_object_section() {
        let input = r#"EventObject "SNDxDDKN" {
            ObjectId 35,
            EventTrack 1 {
                12083,
            }
        }
        "#;

        parse_dbg(input);
    }

    #[test]
    fn parse_camera_section() {
        let input = r#"Camera "Camera01" {
            FieldOfView 0.785398,
            FarClip 10000,
            NearClip 1,
            Position { 106.874, -18.4191, 134.402 },
            Target {
                Position { 7.57606, 1.70948, 123.683 },
            }
        }
        "#;

        parse_dbg(input);
    }
}
