#![feature(map_into_keys_values)]
use regex::Regex;
use std::collections::HashMap;

pub fn parse_file(s: &str) -> Vec<String> {
    println!("Parsing a file of {} characters", s.len());
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(s)
        .map(
            |w| {
                w.as_str()
                    .to_lowercase() // Should this be optional?
                    .replace("_", "") // It's an Alice thing
                    .replace("_", "")
            }, // Closing too
        )
        .collect()
}

fn generate_scores(words: &[String]) -> HashMap<(String, String), usize> {
    println!("Generating scores for {} sequences", words.len());
    let mut prediction_map: HashMap<(String, String), usize> = HashMap::new();
    words.windows(2).for_each(|pair| {
        *prediction_map
            .entry((pair[0].clone(), pair[1].clone()))
            .or_insert(0) += 1;
    });
    prediction_map
}

fn group_wordpredictions(
    predictions_hm: HashMap<(String, String), usize>,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut hm: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for ((first_word, second_word), score) in predictions_hm {
        let entry = hm.entry(first_word).or_insert(vec![]);
        entry.push((second_word, score));
    }
    // Sort the items by score
    let mut hm_sorted: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (first_word, arr) in hm {
        let mut sorted = arr.clone();
        sorted.sort_by(|(_a1, a2), (_b1, b2)| b2.cmp(a2));
        hm_sorted.insert(first_word, sorted);
    }
    hm_sorted
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
        println!(
            "{}",
            word_a
                .iter()
                .map(|(w, s)| format!("{}:{}", w, s))
                .collect::<Vec<String>>()
                .join(",")
        );
        assert_eq!(word_a.len(), 37);
        assert_eq!(word_predictions.len(), 610);
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
