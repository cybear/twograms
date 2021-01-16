use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct WordProposal<'a> {
    pub word: &'a str,
    pub freq: usize,
}

#[derive(Serialize, Debug, Clone, Hash)]
pub struct WordSequence<'a>(&'a str, &'a str);

impl<'a> PartialEq for WordSequence<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<'a> Eq for WordSequence<'a> {}

pub fn parse_line<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| c.is_whitespace())
        .map(|word| word.trim())
        .filter(|word| word.len() > 0)
        .collect()
}

pub fn generate_scores<'a>(sentences: Vec<&'a str>) -> HashMap<WordSequence<'a>, usize> {
    let mut prediction_map = HashMap::new();
    sentences.iter().for_each(|sentence| {
        parse_line(sentence).windows(2).for_each(|word_sequence| {
            *prediction_map
                .entry(WordSequence(&word_sequence[0], &word_sequence[1]))
                .or_insert(0) += &1;
        });
    });
    prediction_map
}

pub fn group_wordpredictions<'a>(
    predictions_hm: HashMap<WordSequence<'a>, usize>,
    keep: usize,
) -> HashMap<&'a str, Vec<WordProposal>> {
    let mut hm = HashMap::new();
    for (word_sequence, score) in predictions_hm {
        hm.entry(word_sequence.0)
            .or_insert(vec![])
            .push(WordProposal {
                word: word_sequence.1,
                freq: score,
            });
    }
    // Sort the items by score descending
    hm.into_iter()
        .map(|(first_word, arr)| {
            let mut sorted = arr.clone();
            sorted.sort_by(|a, b| b.freq.cmp(&a.freq));
            if sorted.len() > keep {
                sorted.resize(
                    keep,
                    WordProposal {
                        word: "foo",
                        freq: 0,
                    }, // This is never used
                );
            }
            (first_word, sorted)
        })
        .collect()
}
