use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twograms::{generate_ngrams, generate_scores, group_wordpredictions, parse_file};

fn criterion_parse(c: &mut Criterion) {
    let bible = include_str!("10900-8.txt");
    c.bench_function("parse_file for bible", |b| {
        b.iter(|| parse_file(black_box(bible)))
    });
}

fn criterion_group_wordpredictions(c: &mut Criterion) {
    let bible = include_str!("10900-8.txt");
    let words = parse_file(bible);
    let scores = generate_scores(words);

    c.bench_function("group_wordpredictions for bible", |b| {
        b.iter(|| group_wordpredictions(black_box(scores.clone()), 100000))
    });
}

fn criterion_generate(c: &mut Criterion) {
    let bible = include_str!("10900-8.txt");
    c.bench_function("Generate for bible", |b| {
        b.iter(|| generate_ngrams(black_box(bible), 100000))
    });
}

criterion_group!(
    benches,
    criterion_parse,
    criterion_group_wordpredictions,
    criterion_generate,
);
criterion_main!(benches);
