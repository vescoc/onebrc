use criterion::{criterion_group, criterion_main, Criterion};

use lazy_static::lazy_static;

use onebrc::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data/weather_stations.csv");
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run f32", |b| {
        b.iter(|| {
            let _ = run::<f32>(&INPUT);
        })
    });

    c.bench_function("run f64", |b| {
        b.iter(|| {
            let _ = run::<f64>(&INPUT);
        })
    });

    c.bench_function("run_par f32", |b| {
        b.iter(|| {
            let _ = run_par::<f32>(&INPUT);
        })
    });

    c.bench_function("run_par f64", |b| {
        b.iter(|| {
            let _ = run_par::<f64>(&INPUT);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
