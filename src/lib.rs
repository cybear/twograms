use std::collections::HashMap;
extern crate wasm_bindgen;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
pub mod parse;
pub mod generate;

#[wasm_bindgen]
pub fn to_json(text: String) -> JsValue {
    let ngrams = generate_ngrams(&text, 5);
    to_value(&ngrams).unwrap()
}

pub fn generate_ngrams<'a>(text: &'a str, keep: usize) -> HashMap<&'a str, Vec<generate::WordProposal>> {
    generate::group_wordpredictions(generate::generate_scores(parse::parse_file(text)), keep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hhgttg() {
        let words = parse::parse_file(include_str!("../benches/hhgttg.txt"));
        let scores = generate::generate_scores(words);
        let word_predictions = generate::group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 585);
        assert_eq!(word_a[0].word, "small");
        assert_eq!(word_predictions.len(), 6045);
    }

    #[test]
    fn test_bible_parser() {
        let words = parse::parse_file(include_str!("../benches/10900-8.txt"));
        let scores = generate::generate_scores(words);
        let word_predictions = generate::group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 1333);
        assert_eq!(word_a[0].word, "man");
    }
}
