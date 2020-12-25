#![feature(map_into_keys_values)]
use std::collections::HashMap;
use rayon::prelude::*;
use regex::Regex;
use std::fmt;

pub struct WordPredictions {
    word: String,
    predictions: Vec<(String, usize)>,
}
impl fmt::Display for WordPredictions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Word: *{}*. Predictions: {}",
            self.word,
            self.predictions.iter().map(|p|
                format!("{} ({})", p.0, p.1)
            ).collect::<Vec<String>>().join(", "),
        )
    }
}

pub fn parse_file(s: &str) -> Vec<String> {
    println!("Parsing a file of {} characters", s.len());
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(s).map(
        |w| w
            .as_str()
            .to_lowercase() // Should this be optional?
            .replace("_", "") // It's an Alice thing
            .replace("_", "") // Closing too
    ).collect()
}

fn generate_scores(words: &[String]) -> HashMap<String, usize> {
    println!("Generating scores for {} sequences", words.len());
    let mut prediction_map: HashMap<String, usize> = HashMap::new();
    let iter = words.windows(2);
    let keys = iter.map(|pair| format!("{}§{}", pair[0], pair[1]));
    let val: usize = 1;
    for key in keys {
        let key = key.clone();
        if !prediction_map.contains_key(key.as_str()) {
            prediction_map.insert(key, val);
        } else {
            let val = prediction_map.get(key.as_str()).unwrap() + 1;
            prediction_map.insert(key, val);
        }
    }
    prediction_map
}

fn clean_scores(map: HashMap<String, usize>, minimum: usize) -> HashMap<String, usize> {
    map.into_iter()
        .filter(|&(_, v)| v >= minimum)
        .collect()
}

fn get_unique_words(words: &Vec<String>) -> Vec<String> {
    println!("Deduping {} words", words.len());
    let mut words_sorted = words.clone();
    words_sorted.sort();
    words_sorted.dedup();
    words_sorted
}

fn keyval_hashmap_to_wordprediction_hashmap(unique_words: Vec<String>, predictions_map: HashMap<String, usize>) -> HashMap<String, WordPredictions> {
    println!("Generating word predictions for {} words", unique_words.len());
    unique_words
        .par_iter()
        .map(|first_word| {
            let prefix = format!("{}§", first_word);
            let second_word_keys = predictions_map
                .keys()
                .filter(|word| word.starts_with(&prefix));
            let mut second_word_scores: Vec<(String, usize)> = second_word_keys
                .map(|key| {
                    let mut split = key.split("§");
                    split.next();
                    let second_word = split.next().unwrap();
                    (second_word.to_string(), predictions_map.get(key).unwrap().clone())}
                )
                .collect();
            second_word_scores.sort_by(|a, b| b.1.cmp(&a.1));
            (
                first_word.to_string(),
                WordPredictions {
                word: first_word.to_string(),
                predictions: second_word_scores,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "I am a fish. No, wait, I am a plant.";

#[test]
fn test_parse_file() {
    let words = parse_file(TESTDATA);
    assert_eq!(words.join(" "), "i am a fish no wait i am a plant");
}

#[test]
fn test_generate_n_grams() {
    let words = parse_file(TESTDATA);
    let two_grams = generate_scores(&words);
    let score1: usize = 1;
    let score2: usize = 2;
    assert_eq!(two_grams.get("i§am").unwrap(), &score2);
    assert_eq!(two_grams.get("am§a").unwrap(), &score2);
    assert_eq!(two_grams.get("a§fish").unwrap(), &score1);
}

#[test]
fn test_alice() {
    let words = parse_file(include_str!("alice.txt"));
    let unique_words = get_unique_words(&words);
    let two_grams = generate_scores(&words);
    let word_predictions = keyval_hashmap_to_wordprediction_hashmap(unique_words, two_grams);
    let word_a = word_predictions.get("a").unwrap();
    assert_eq!(word_a.word, "a");
    assert_eq!(word_a.predictions.len(), 37);    
    assert_eq!(word_predictions.len(), 610);
}

#[test]
fn test_alice_hm() {
    let words = parse_file(include_str!("alice.txt"));
    let unique_words = get_unique_words(&words);
    let two_grams = generate_scores(&words);
    let word_predictions = keyval_hashmap_to_wordprediction_hashmap(unique_words, two_grams);
    let word_a = word_predictions.get("a").unwrap();
    assert_eq!(word_a.word, "a");
    assert_eq!(word_a.predictions.len(), 37);
    assert_eq!(word_predictions.len(), 610);
}

#[test]
fn test_bible_parser() {
    let words = parse_file(include_str!("10900-8.txt"));
    assert_eq!(words.len(), 858338);
    let unique_words = get_unique_words(&words);
    let two_grams = generate_scores(&words);
    let two_grams_score_above_1 = clean_scores(two_grams, 2);
    let word_predictions = keyval_hashmap_to_wordprediction_hashmap(unique_words, two_grams_score_above_1);
    let word_a = word_predictions.get("a").unwrap();
    assert_eq!(word_a.word, "a");
    assert_eq!(word_a.predictions.len(), 795);
}
}
