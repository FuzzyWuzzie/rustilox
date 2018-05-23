#[macro_use]
extern crate criterion;
extern crate rustilox;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scan strings", |b| {
        let src: String = "\"I am a string\"".to_string();
        b.iter(|| {
            rustilox::interpret(&src)
        })
    });

    c.bench_function("scan numbers", |b| {
        let src: String = "3.14159263".to_string();
        b.iter(|| {
            rustilox::interpret(&src)
        })
    });

    c.bench_function("scan keywords", |b| {
        let src: String = "while true false for and print".to_string();
        b.iter(|| {
            rustilox::interpret(&src)
        })
    });

    c.bench_function("scan identifiers", |b| {
        let src: String = "Leonardo Donatello Michelangelo Raphael".to_string();
        b.iter(|| {
            rustilox::interpret(&src)
        })
    });

    c.bench_function("scan comments", |b| {
        let src: String = "// I am a comment!".to_string();
        b.iter(|| {
            rustilox::interpret(&src)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);