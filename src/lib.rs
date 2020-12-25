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
        let predictions = self.predictions
            .iter()
            .map(
                |p| format!("{} ({})", p.0, p.1
            ))
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "Word: *{}*. Predictions: {}",
            self.word,
            predictions,
        )
    }
}
struct WordScore {
    word: String,
    second_word: String,
    score: usize,
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

fn generate_scores(words: &[String]) -> Vec<WordScore> {
    println!("Generating scores for {} sequences", words.len());
    let mut prediction_map: HashMap<String, usize> = HashMap::new();
    words
        .windows(2)
        .map(
            |pair| format!("{}§{}", pair[0], pair[1])
        )
        .for_each(
            |key| *prediction_map.entry(key).or_insert(0) += 1
        );
    // At least only do this ONCE
    prediction_map.iter().map(|(key, score)| {
        prediction_tuple_to_word_score(key, *score)
    }).collect()
}

fn prediction_tuple_to_word_score(key: &String, score: usize) -> WordScore {
    let mut split = key.split("§");
        let word = split.next().unwrap().to_string();
        let second_word = split.next().unwrap().to_string();
        WordScore {
            word,
            second_word,
            score,
        }
}

fn get_unique_words(words: &Vec<String>) -> Vec<String> {
    println!("Deduping {} words", words.len());
    let mut words_sorted = words.clone();
    words_sorted.sort();
    words_sorted.dedup();
    words_sorted
}

fn keyval_hashmap_to_wordprediction_hashmap(
    unique_words: Vec<String>,
    predictions: Vec<WordScore>,
) -> HashMap<String, WordPredictions> {
    println!("Generating word predictions for {} words", unique_words.len());
    unique_words
        .par_iter()
        .map(|first_word| {
            let mut second_word_scores: Vec<&WordScore> = predictions
                .iter()
                .filter(|word_score| word_score.word == *first_word)
                .collect();
            second_word_scores.sort_by(|a, b| b.score.cmp(&a.score));
            (
                first_word.to_string(),
                WordPredictions {
                    word: first_word.to_string(),
                    predictions: second_word_scores
                        .iter()
                        .map(|word_score| (word_score.second_word.clone(), word_score.score))
                        .collect(),
                }
            )
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
fn test_bible_parser() {
    let words = parse_file(include_str!("10900-8.txt"));
    assert_eq!(words.len(), 858338);
    let unique_words = get_unique_words(&words);
    let two_grams = generate_scores(&words);
    let word_predictions = keyval_hashmap_to_wordprediction_hashmap(unique_words, two_grams);
    let word_a = word_predictions.get("a").unwrap();
    assert_eq!(word_a.word, "a");
    assert_eq!(word_a.predictions.len(), 1335);
}
}
