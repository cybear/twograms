use std::collections::HashMap;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value};

#[wasm_bindgen]
pub fn to_json(text: String) -> JsValue {
    let ngrams = generate_ngrams(&text, 5);
    to_value(&ngrams).unwrap()
}


pub fn generate_ngrams<'a>(text: &'a str, keep: usize) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    group_wordpredictions(generate_scores(parse_file(text)), keep)
}

fn parse_line<'a>(s: &'a str) -> Vec<&'a str> {
    s
        .split(|c: char| c.is_whitespace())
        .map(|word| word.trim())
        .filter(|word| word.len() > 0)
        .collect()
}

pub fn parse_file<'a>(s: &'a str) -> Vec<&'a str> {
    s
    .split(|c: char| c.is_ascii_punctuation())
    .map(|sentence| sentence.trim())
    .filter(|sentence| sentence.len() > 0)
    .collect()
}

pub fn generate_scores<'a>(sentences: Vec<&'a str>) -> HashMap<(&'a str, &'a str), usize> {
    let mut prediction_map: HashMap<(&'a str, &'a str), usize> = HashMap::new();
    sentences.iter().for_each(|sentence| {
        parse_line(sentence)
            .windows(2)
            .for_each(|word_sequence| {
            *prediction_map
                .entry((
                    &word_sequence[0],
                    &word_sequence[1],
                ))
                .or_insert(0) += &1;
        });
    });
    prediction_map
}

pub fn group_wordpredictions<'a>(
    predictions_hm: HashMap<(&'a str, &'a str), usize>,
    keep: usize,
) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    let mut hm: HashMap<&'a str, Vec<(&'a str, usize)>> = HashMap::new();
    for (word_sequence, score) in predictions_hm {
        hm.entry(word_sequence.0)
            .or_insert(vec![])
            .push((word_sequence.1, score));
    }
    // Sort the items by score descending
    hm.into_iter()
        .map(|(first_word, arr)| {
            let mut sorted = arr.clone();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
            if sorted.len() > keep {
                sorted.resize(
                    keep,
                    ("foo", 0), // This is never used
                );
            }
            (first_word, sorted)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "I am a fish. No, wait, I am a plant.";

    #[test]
    fn test_parse_file() {
        let sentences = parse_file(TESTDATA);
        assert_eq!(sentences[0], "I am a fish");
        assert_eq!(sentences[1], "No");
        assert_eq!(sentences[2], "wait");
        assert_eq!(sentences[3], "I am a plant");
    }

    #[test]
    fn test_hhgttg() {
        let words = parse_file(include_str!("../benches/hhgttg.txt"));
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 585);
        assert_eq!(word_a[0].0, "small");
        assert_eq!(word_predictions.len(), 6045);
    }

    #[test]
    fn test_bible_parser() {
        let words = parse_file(include_str!("../benches/10900-8.txt"));
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 1333);
        assert_eq!(word_a[0].0, "man");
    }
}
