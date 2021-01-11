// use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Clone, Serialize, Debug)]
pub struct WordProposal(String, usize);
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
pub struct WordSequence(String, String);

pub fn generate_ngrams(text: &str, keep: usize) -> HashMap<String, Vec<WordProposal>> {
    group_wordpredictions(generate_scores(parse_file(text)), keep)
}

fn parse_line(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| w.to_lowercase()).collect()
}

pub fn parse_file(s: &str) -> impl Iterator<Item=Vec<String>> + '_ {
    s
    .split(|c: char| c.is_ascii_punctuation())
    .map(|s| parse_line(s))
}

pub fn generate_scores(sentences: impl Iterator<Item=Vec<String>>) -> HashMap<WordSequence, usize> {
    let mut prediction_map: HashMap<WordSequence, usize> = HashMap::new();
    for sentence in sentences {
        sentence.windows(2).for_each(|word_sequence| {
            *prediction_map
                .entry(WordSequence(
                    word_sequence[0].clone(),
                    word_sequence[1].clone(),
                ))
                .or_insert(0) += 1;
        });
    }
    prediction_map
}

pub fn group_wordpredictions(
    predictions_hm: HashMap<WordSequence, usize>,
    keep: usize,
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
            if sorted.len() > keep {
                sorted.resize(
                    keep,
                    WordProposal("foo".into(), 0), // This is never used
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
        let mut sentences = parse_file(TESTDATA);
        assert_eq!(sentences.next().unwrap().join(" "), "i am a fish");
        assert_eq!(sentences.next().unwrap().join(" "), "no");
        assert_eq!(sentences.next().unwrap().join(" "), "wait");
        assert_eq!(sentences.next().unwrap().join(" "), "i am a plant");
    }

    #[test]
    fn test_hhgttg() {
        let words = parse_file(include_str!("../benches/hhgttg.txt"));
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 605);
        assert_eq!(word_a[0].0, "small");
        assert_eq!(word_predictions.len(), 5595);
    }

    #[test]
    fn test_bible_parser() {
        let words = parse_file(include_str!("../benches/10900-8.txt"));
        // assert_eq!(words.collect().len(), 157387);
        let scores = generate_scores(words);
        let word_predictions = group_wordpredictions(scores, 1000000);
        let word_a = word_predictions.get("a").unwrap();
        assert_eq!(word_a.len(), 1333);
        assert_eq!(word_a[0].0, "man");
    }
}
