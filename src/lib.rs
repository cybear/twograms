use std::collections::HashMap;
use regex::Regex;

pub struct WordPredictions {
    word: String,
    predictions: Vec<(String, usize)>,
}

// HashMap<String, usize>, // These would probably be useful to sort according to score

/**
 * Import:
 * - Each word should have a hashmap 
 * - Its preceding word gets a score +1 in that hashmap
 * - 
 */

pub fn parse_file(s: &str) -> Vec<String> {
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(s).map(
        |w| w.as_str().to_lowercase()
    ).collect()
}

fn generate_scores(words: &[String]) -> HashMap<String, usize> {
    let mut prediction_map: HashMap<String, usize> = HashMap::new();
    let iter = words.windows(2);
    let keys = iter.map(|pair| format!("{}:{}", pair[0], pair[1]));
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

fn keyval_hashmap_to_wordpredictions(predictions_map: HashMap<String, usize>) -> Vec<WordPredictions> {
    let mut words: Vec<&str> = predictions_map.keys()
        .map(|key| {
            let mut split = key.split(":");
            split.next().unwrap()
        })
        .collect();
    words.dedup();
    words.iter().map(|first_word| {
        let prefix = format!("{}:", first_word);
        let second_word_keys = predictions_map.keys()
            .filter(|word| word.starts_with(&prefix));
        let second_word_scores: Vec<(String, usize)> = second_word_keys.map(
            |key| (key.clone(), predictions_map.get(key).unwrap().clone())
        ).collect(); // TODO: sort by score descending
        WordPredictions {
            word: first_word.to_string(),
            predictions: second_word_scores,
        }
    }).collect()
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
    assert_eq!(two_grams.get("i:am").unwrap(), &score2);
    assert_eq!(two_grams.get("am:a").unwrap(), &score2);
    assert_eq!(two_grams.get("a:fish").unwrap(), &score1);
}

#[test]
fn test_keyval_hashmap_to_wordpredictions() {
    let words = parse_file(TESTDATA);
    let two_grams = generate_scores(&words);
    let word_predictions = keyval_hashmap_to_wordpredictions(two_grams);
    assert_eq!(word_predictions[0].word, "i");
}
}