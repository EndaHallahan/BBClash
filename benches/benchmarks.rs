#[macro_use]
extern crate criterion;

use criterion::{Criterion};
use bbclash::bbcode_to_html;

mod texts;
pub use texts::{BG_PON, EV_ESY, QN_RAR, WH_DNC, TS_TWN, TN_CMT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Bench 1 (43,858 words)", move |b| b.iter(|| bbcode_to_html(BG_PON)));
    c.bench_function("Bench 2 (17,659 words)", move |b| b.iter(|| bbcode_to_html(EV_ESY)));
    c.bench_function("Bench 3 (14,086 words)", move |b| b.iter(|| bbcode_to_html(QN_RAR)));
    c.bench_function("Bench 4 (8,800 words)", move |b| b.iter(|| bbcode_to_html(WH_DNC)));
    c.bench_function("Bench 5 (2,379 words)", move |b| b.iter(|| bbcode_to_html(TS_TWN)));
    c.bench_function("Bench 6 (2 words)", move |b| b.iter(|| bbcode_to_html(TN_CMT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

