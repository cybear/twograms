use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twograms::{generate_ngrams, generate_scores, group_wordpredictions, parse_file};

fn criterion_generate(c: &mut Criterion) {
    let bible = include_str!("10900-8.txt");
    let words = parse_file(&bible);
    let scores = generate_scores(parse_file(&bible));

    let mut group = c.benchmark_group("twograms lib");
    // group.significance_level(0.1).sample_size(10);

    group.bench_function("parse_file for bible", |b| {
        b.iter(|| parse_file(black_box(bible)))
    });
    group.bench_function("generate_scores for bible", |b| {
        b.iter(|| generate_scores(black_box(words.clone())))
    });
    group.bench_function("group_wordpredictions for bible", |b| {
        b.iter(|| group_wordpredictions(black_box(scores.clone()), 100000))
    });
    group.bench_function("Generate for bible", |b| {
        b.iter(|| generate_ngrams(black_box(bible), 100000))
    });
    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default(); //.sample_size(20);
    targets = criterion_generate,
}

criterion_main!(benches);
