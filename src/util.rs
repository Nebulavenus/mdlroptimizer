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
    (result)
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
            let start = line[0] - difference;
            let mut end = line[1] - difference;
            let old_value = &result.clone()[start..end];
            result.replace_range(start..end, "");

            //dbg!(&new_value);
            end -= old_value.len();
            result.replace_range(start..end, new_value.to_string().as_str());
            difference += end - start;
            //dbg!(difference);
            //dbg!(idx);
        })
        .for_each(drop);
    result
}