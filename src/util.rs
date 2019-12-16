use regex::Regex;

// Regex: \/\/[^\n]* or simply //.+ in rust
pub fn remove_comments(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"//[^\n]*").unwrap();
    }
    RE.replace_all(&text, "").to_string()
}

pub fn remove_redundant_lines(input: String, spans: Vec<[usize; 2]>) -> (String, usize) {
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
    (result, difference)
}

pub fn replace_values_at_spans(input: String, spans: Vec<([usize; 2], u32)>, diff: usize) -> (String, usize) {
    let mut result = input;
    let mut difference = diff;
    spans
        .iter()
        .enumerate()
        .map(|(idx, span)| {
            let (line, value) = span;
            let start = line[0] - difference;
            let mut end = line[1] - difference;
            result.replace_range(start..end, "");

            //end += value.to_string().as_str().len();
            result.replace_range(start..end, value.to_string().as_str());
            difference += line[1] - line[0] + value.to_string().as_str().len();
            dbg!(difference);
            //dbg!(idx);
        })
        .for_each(drop);
    (result, difference)
}