use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use textdistance::algorithms::base::{TextDistance, TextSimilarity};
use textdistance::*;

fn bench_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Distance Algorithms");

    let inputs = vec![
        ("kitten", "sitting", "Short (6 chars)"),
        ("the quick brown fox jumps over the lazy dog", "the quick brown fox jumped over the lazy dogs", "Medium (45 chars)"),
        (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliquyam erat.",
            "Long (130 chars)"
        )
    ];

    for (s1, s2, id) in inputs {
        // Levenshtein
        let lev = Levenshtein::new();
        group.bench_with_input(BenchmarkId::new("Levenshtein", id), &(s1, s2), |b, &(s1, s2)| {
            b.iter(|| lev.distance(black_box(s1), black_box(s2)));
        });

        // JaroWinkler
        let jw = JaroWinkler::new();
        group.bench_with_input(BenchmarkId::new("JaroWinkler", id), &(s1, s2), |b, &(s1, s2)| {
            b.iter(|| jw.similarity(black_box(s1), black_box(s2)));
        });

        // Jaccard
        let jaccard = Jaccard::new();
        group.bench_with_input(BenchmarkId::new("Jaccard", id), &(s1, s2), |b, &(s1, s2)| {
            b.iter(|| jaccard.similarity(black_box(s1), black_box(s2)));
        });

        // RatcliffObershelp
        let ro = RatcliffObershelp::new();
        group.bench_with_input(BenchmarkId::new("RatcliffObershelp", id), &(s1, s2), |b, &(s1, s2)| {
            b.iter(|| ro.similarity(black_box(s1), black_box(s2)));
        });

        // ArithNCD (Compression)
        let ncd = ArithNcd::new();
        group.bench_with_input(BenchmarkId::new("ArithNCD", id), &(s1, s2), |b, &(s1, s2)| {
            b.iter(|| ncd.distance(black_box(s1), black_box(s2)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_algorithms);
criterion_main!(benches);
