#[cfg(test)]
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
