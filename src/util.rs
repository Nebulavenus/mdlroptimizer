use regex::Regex;

// Regex: \/\/[^\n]* or simply //.+ in rust
pub fn remove_comments(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"//[^\n]*").unwrap();
    }
    RE.replace_all(&text, "").to_string()
}

/*
type Span = [usize; 2];

impl From<pest::Span> for Span {
    fn from(span: pest::Span) -> Self {
        [span.start(), span.end()]
    }
}

impl From<Vec<pest::Span>> for Vec<Span> {
    fn from(spans: Vec<pest::Span>) -> Self {
        spans
            .map(|span| [span.start(), span.end()])
            .collect()
    }
}
*/
