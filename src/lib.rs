#![feature(map_into_keys_values)]
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct WordProposal(String, usize);
#[derive(Clone, PartialEq, Eq, Hash)]
struct WordSequence(String, String);

pub fn generate_ngrams(text: &str) -> HashMap<String, Vec<WordProposal>> {
    group_wordpredictions(generate_scores(&parse_file(text)))
}

fn parse_file(s: &str) -> Vec<String> {
    println!("Parsing a file of {} characters", s.len());
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(s)
        .map(
            |w| {
                w.as_str()
                    .to_lowercase() // Should this be optional?
                    .replace("_", "") // It's an Alice thing
                    .replace("_", "") // Closing too
            }, //
        )
        .collect()
}

fn generate_scores(words: &[String]) -> HashMap<WordSequence, usize> {
    println!("Generating scores for {} sequences", words.len());
    let mut prediction_map: HashMap<WordSequence, usize> = HashMap::new();
    words.windows(2).for_each(|word_sequence| {
        *prediction_map
            .entry(WordSequence(word_sequence[0].clone(), word_sequence[1].clone()))
            .or_insert(0) += 1;
    });
    prediction_map
}

fn group_wordpredictions(
    predictions_hm: HashMap<WordSequence, usize>,
) -> HashMap<String, Vec<WordProposal>> {
    let mut hm: HashMap<String, Vec<WordProposal>> = HashMap::new();
    for (word_sequence, score) in predictions_hm {
        hm.entry(word_sequence.0)
            .or_insert(vec![])
            .push(WordProposal(word_sequence.1, score));
    }
    // Sort the items by score descending
    hm.into_iter()
        .map(|(first_word, arr)| {
            let mut sorted = arr.clone();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
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
        let words = parse_file(TESTDATA);
        assert_eq!(words.join(" "), "i am a fish no wait i am a plant");
    }

    #[test]
    fn test_alice() {
        let words = parse_file(include_str!("alice.txt"));
        let scores = generate_scores(&words);
        let word_predictions = group_wordpredictions(scores);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 37);
        assert_eq!(word_predictions.len(), 610);
        assert_eq!(word_a[0].0, "little");
    }

    #[test]
    fn test_hhgttg() {
        let words = parse_file(include_str!("hhgttg.txt"));
        let scores = generate_scores(&words);
        let word_predictions = group_wordpredictions(scores);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 608);
        assert_eq!(word_a[0].0, "small");
        assert_eq!(word_predictions.len(), 6138);
    }

    #[test]
    fn test_bible_parser() {
        let words = parse_file(include_str!("10900-8.txt"));
        assert_eq!(words.len(), 858338);
        let scores = generate_scores(&words);
        let word_predictions = group_wordpredictions(scores);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 1335);
        assert_eq!(word_a[0].0, "man");
    }
}
