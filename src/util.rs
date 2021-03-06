use regex::Regex;

// Regex: \/\/[^\n]* or simply //.+ in rust
pub fn remove_comments(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"//[^\n]*").unwrap();
    }
    RE.replace_all(&text, "").to_string()
}

pub fn remove_tabs_newlines(text: &str) -> String {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"\t|\r|\f").unwrap();
    }
    RE1.replace_all(&text, "").to_string()
}

pub fn remove_redundant_lines(input: String, spans: Vec<[usize; 2]>) -> String {
    let mut result = input;
    let mut difference = 0usize;
    spans
        .iter()
        .enumerate()
        .map(|(_idx, line)| {
            let start = line[0] - difference;
            let end = line[1] - difference;
            result.replace_range(start..end, "");
            difference += line[1] - line[0];
        })
        .for_each(drop);
    result
}

pub fn replace_values_at_spans(input: String, spans: Vec<([usize; 2], u32)>) -> String {
    let mut result = input;
    let mut difference = 0usize;
    spans
        .iter()
        .enumerate()
        .map(|(_idx, span)| {
            let (line, new_value) = span;
            let mut start = line[0] - difference;
            let mut end = line[1] - difference;
            let old_value = &result.clone()[start..end];

            let mut whitespaces = String::with_capacity(old_value.len());
            for _ in 0..old_value.len() {
                whitespaces.push_str(" ");
            }
            result.replace_range(start..end, whitespaces.as_str());

            let corr = old_value.len() - new_value.to_string().len();
            end -= corr;

            result.replace_range(start..end, new_value.to_string().as_str());
        })
        .for_each(drop);
    result
}

pub fn replace_interp_type_at_spans(input: String, spans: Vec<([usize; 2])>) -> String {
    let mut result = input;
    let mut difference = 0usize;
    spans
        .iter()
        .enumerate()
        .map(|(_idx, span)| {
            let line = span;
            let mut start = line[0] - difference;
            let mut end = line[1] - difference;
            let old_str = &result.clone()[start..end];

            let corr = old_str.len() - "Linear".len();
            end -= corr;

            result.replace_range(start..end, "Linear");
        })
        .for_each(drop);
    result
}
