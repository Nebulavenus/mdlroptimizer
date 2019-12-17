use regex::Regex;

// Regex: \/\/[^\n]* or simply //.+ in rust
pub fn remove_comments(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"//[^\n]*").unwrap();
    }
    RE.replace_all(&text, "").to_string()
}

pub fn remove_redundant_lines(input: String, spans: Vec<[usize; 2]>) -> String {
    let mut result = input;
    let mut difference = 0usize;
    spans
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let start = line[0] - difference;
            let end = line[1] - difference;
            result.replace_range(start..end, "");
            difference += line[1] - line[0];
            //dbg!(difference);
            //dbg!(idx);
        })
        .for_each(drop);
    result
}

// TODO(nv): audit this part of code VERY careful
pub fn replace_values_at_spans(input: String, spans: Vec<([usize; 2], u32)>) -> String {
    let mut result = input;
    let mut difference = 0usize;
    spans
        .iter()
        .enumerate()
        .map(|(idx, span)| {
            let (line, new_value) = span;
            dbg!(new_value);
            let mut start = line[0] - difference;
            let mut end = line[1] - difference;
            let old_value = &result.clone()[start..end];
            dbg!(old_value);
            //result.replace_range(start..end, "");
            //start -= old_value.len();
            //end -= old_value.len();
            let corr = old_value.len() - new_value.to_string().len();
            dbg!(corr);
            //end += corr;

            //dbg!(&new_value);
            //end -= old_value.len();
            result.replace_range(start..end, new_value.to_string().as_str());
            //difference += end - start;
            if corr != 0 {
                difference += corr;
            }
            //dbg!(difference);
            //dbg!(idx);
        })
        .for_each(drop);
    result
}

pub fn combine_spans_and_value(spans: Vec<(String, [usize;2])>, values: Vec<(String, u32)>)
    -> Vec<([usize; 2], u32)> {
    let mut result = Vec::<([usize; 2], u32)>::new();
    for (name, new_value) in values {
        let mut some_span: Option<[usize; 2]> = None;
        for (bone_name, translation_span) in spans.clone() {
            if name == bone_name {
                some_span = Some(translation_span);
                break;
            }
            some_span = None;
        }
        match some_span {
            Some(span) => {
                result.push((span, new_value));
            },
            None => ()
        }
    }
    result
}