use advent_of_code::solve;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

pub fn criterion_benchmark(c: &mut Criterion) {
    #![allow(clippy::unwrap_used)]
    for year in 2015..=2022 {
        let start_day = 1;
        let end_day = 25;
        for day in start_day..=end_day {
            let input_path = format!("src/year{year}/day{day:02}_input.txt");
            let input = read_to_string(input_path).unwrap();

            for part in 1..=(if day == 25 { 1 } else { 2 }) {
                let benchmark_name = format!("{year}_{day:02}_{part}");
                c.bench_function(&benchmark_name, |b| {
                    b.iter(|| solve(year, day, part, &input));
                });
            }
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::new(1, 0))
        .nresamples(10_000)
        .measurement_time(std::time::Duration::new(3, 0));
    targets = criterion_benchmark
}

criterion_main!(benches);
