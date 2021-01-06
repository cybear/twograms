use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize, Debug)]
pub struct WordProposal(String, usize);
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
pub struct WordSequence(String, String);

pub fn to_json(text: &str) -> String {
    let ngrams = generate_ngrams(text);
    serde_json::to_string(&ngrams).unwrap()
}

pub fn generate_ngrams(text: &str) -> HashMap<String, Vec<WordProposal>> {
    group_wordpredictions(generate_scores(parse_file(text)))
}

pub fn parse_file(s: &str) -> Vec<Vec<String>> {
    s.as_parallel_string()
        .par_split(|c: char| c.is_ascii_punctuation())
        .map(|s| s.split_whitespace().map(|w| w.to_lowercase()).collect())
        .collect()
}

pub fn generate_scores(sentences: Vec<Vec<String>>) -> HashMap<WordSequence, usize> {
    let mut prediction_map: HashMap<WordSequence, usize> = HashMap::new();
    sentences.iter().for_each(|sentence| {
        sentence.windows(2).for_each(|word_sequence| {
            *prediction_map
                .entry(WordSequence(
                    word_sequence[0].clone(),
                    word_sequence[1].clone(),
                ))
                .or_insert(0) += 1;
        });
    });
    prediction_map
}

pub fn group_wordpredictions(
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
        let sentences = parse_file(TESTDATA);
        assert_eq!(sentences[0].join(" "), "i am a fish");
        assert_eq!(sentences[1].join(" "), "no");
        assert_eq!(sentences[2].join(" "), "wait");
        assert_eq!(sentences[3].join(" "), "i am a plant");
    }

    #[test]
    fn test_hhgttg() {
        let words = parse_file(include_str!("hhgttg.txt"));
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 605);
        assert_eq!(word_a[0].0, "small");
        assert_eq!(word_predictions.len(), 5595);
    }

    #[test]
    fn test_bible_parser() {
        let words = parse_file(include_str!("10900-8.txt"));
        assert_eq!(words.len(), 157387);
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 1333);
        assert_eq!(word_a[0].0, "man");
    }
}
