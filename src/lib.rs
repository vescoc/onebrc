#![deny(clippy::pedantic)]

use std::collections::HashMap;
use std::{fmt, ops, str, thread};

/// Run
///
/// # Panics
/// Panic if invalid input
pub fn run_filter<F, const S: usize>(
    input: &str,
    f: impl Fn(&str) -> bool,
) -> impl Iterator<Item = (&str, (u16, F, F, F))>
where
    F: str::FromStr,
    <F as str::FromStr>::Err: fmt::Debug,
    F: PartialOrd,
    F: Copy,
    F: ops::AddAssign,
    F: ops::Div<Output = F>,
    F: From<u16>,
{
    let mut map = HashMap::with_capacity(S);
    for line in input.lines() {
        if let Some((location, value)) = line.split_once(';') {
            if f(location) {
                let value = value.parse::<F>().expect("invalid input");
                map.entry(location)
                    .and_modify(|(count, min, max, total)| {
                        *count += 1;
                        if *min > value {
                            *min = value;
                        }
                        if *max < value {
                            *max = value;
                        }
                        *total += value;
                    })
                    .or_insert((1, value, value, value));
            }
        }
    }

    map.into_iter().map(|(location, (count, min, max, total))| {
        (location, (count, min, max, total / F::from(count)))
    })
}

pub fn run<F>(input: &str) -> impl Iterator<Item = (&str, (u16, F, F, F))>
where
    F: str::FromStr,
    <F as str::FromStr>::Err: fmt::Debug,
    F: PartialOrd,
    F: Copy,
    F: ops::AddAssign,
    F: ops::Div<Output = F>,
    F: From<u16>,
{
    run_filter::<_, 50_000>(input, |_| true)
}

/// Run
///
/// # Panics
/// Panic if invalid input
pub fn run_par<F>(input: &str) -> impl Iterator<Item = (&str, (u16, F, F, F))>
where
    F: str::FromStr,
    <F as str::FromStr>::Err: fmt::Debug,
    F: PartialOrd,
    F: Copy,
    F: ops::AddAssign,
    F: ops::Div<Output = F>,
    F: From<u16>,
    F: Send,
{
    const SIZE: usize = 50_000;

    // ABCDEFGHIJKLMNOPQRSTUVWXYZ
    thread::scope(|s| {
        let h_0 = s.spawn(|| run_filter::<_, SIZE>(input, |location| location < "F"));
        let h_1 =
            s.spawn(|| run_filter::<_, SIZE>(input, |location| ("F".."M").contains(&location)));
        let h_2 =
            s.spawn(|| run_filter::<_, SIZE>(input, |location| ("M".."U").contains(&location)));

        run_filter::<_, SIZE>(input, |location| location >= "U")
            .chain(h_0.join().unwrap())
            .chain(h_1.join().unwrap())
            .chain(h_2.join().unwrap())
    })
}
