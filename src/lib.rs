use std::collections::HashMap;
extern crate wasm_bindgen;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
pub mod generate;
pub mod parse;
pub mod test;

#[wasm_bindgen]
pub fn to_json(text: String) -> JsValue {
    let ngrams = generate_ngrams(&text, 5);
    to_value(&ngrams).unwrap()
}

pub fn generate_ngrams<'a>(
    text: &'a str,
    keep: usize,
) -> HashMap<&'a str, Vec<generate::WordProposal>> {
    generate::group_wordpredictions(generate::generate_scores(parse::parse_file(text)), keep)
}
