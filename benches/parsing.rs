use bevy_mod_bbcode::parser::parse_bbcode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn small_raw_text(c: &mut Criterion) {
    c.bench_function("small raw text", |b| {
        b.iter(|| {
            parse_bbcode(black_box(
                "A simple small text string with just normal characters",
            ))
        })
    });
}

criterion_group!(benches, small_raw_text);
criterion_main!(benches);
