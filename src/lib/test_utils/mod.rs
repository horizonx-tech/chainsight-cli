use std::fmt;

use proc_macro2::TokenStream;

pub struct SrcString {
    value: String,
}

impl From<&TokenStream> for SrcString {
    fn from(value: &TokenStream) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl fmt::Display for SrcString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", simple_src_format(&self.value))
    }
}

pub fn simple_src_format(src: &str) -> String {
    let regex = regex::Regex::new(r" (?:(::|[,.!<>:;(\[])) ?").unwrap();
    let res = regex.replace_all(src, "$1");

    let regex = regex::Regex::new(r"(,)").unwrap();
    let res = regex.replace_all(&res, "$1 ");

    let regex = regex::Regex::new(r" ?(\}[^;])").unwrap();
    let res = regex.replace_all(&res, "\n$1");

    let regex = regex::Regex::new(r"(}) ").unwrap();
    let res = regex.replace_all(&res, "$1\n\n");

    let regex = regex::Regex::new(r"([;\]]|\{) ?").unwrap();
    let res = regex.replace_all(&res, "$1\n");

    res.to_string()
}
