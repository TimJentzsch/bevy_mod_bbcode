use std::fs;

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

pub fn small_text_with_simple_formatting(c: &mut Criterion) {
    c.bench_function("small text with simple formatting", |b| {
        b.iter(|| {
            parse_bbcode(black_box(
                "A simple text with [b]bold[/b] and [i]italic[i] elements",
            ))
        })
    });
}

criterion_group!(
    benches_small,
    small_raw_text,
    small_text_with_simple_formatting
);

pub fn large_raw_text(c: &mut Criterion) {
    let input = &fs::read_to_string("benches/input/sample_5000_raw.bbcode").unwrap();

    c.bench_function("large raw text", |b| {
        b.iter(|| parse_bbcode(black_box(input)))
    });
}

pub fn large_text_with_simple_formatting(c: &mut Criterion) {
    let input = &fs::read_to_string("benches/input/sample_5000_simple.bbcode").unwrap();

    c.bench_function("large text with simple formatting", |b| {
        b.iter(|| parse_bbcode(black_box(input)))
    });
}

criterion_group!(
    benches_large,
    large_raw_text,
    large_text_with_simple_formatting
);

criterion_main!(benches_small, benches_large);
