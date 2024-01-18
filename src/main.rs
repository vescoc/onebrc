use lazy_static::lazy_static;

use std::time::Instant;

use onebrc::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data/weather_stations.csv");
}

fn main() {
    let now = Instant::now();
    for (location, (_, min, max, mean)) in run_par::<f32>(&INPUT) {
        println!("{location};{min:.1};{max:.1};{mean:.1}");
    }

    let elapsed = now.elapsed();

    eprintln!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
