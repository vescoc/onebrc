#![deny(clippy::pedantic)]

use std::collections::HashMap;
use std::{fmt, ops, str, sync::mpsc, thread};

/// Run
///
/// # Panics
/// Panic if invalid input
fn map_phase<'a, F, const S: usize>(
    lines: impl Iterator<Item = &'a str>,
) -> HashMap<&'a str, (u16, F, F, F)>
where
    F: str::FromStr,
    <F as str::FromStr>::Err: fmt::Debug,
    F: PartialOrd,
    F: Copy,
    F: ops::AddAssign,
    F: From<u16>,
{
    let mut map = HashMap::with_capacity(S);
    for line in lines {
        if let Some((location, value)) = line.split_once(';') {
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

    map
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
    map_phase::<_, 50_000>(input.lines())
        .into_iter()
        .map(|(location, (count, min, max, total))| {
            (location, (count, min, max, total / F::from(count)))
        })
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
    const BATCH_SIZE: usize = 1_024 * 20;

    let (out_tx, out_rx) = mpsc::channel();
    thread::scope(|s| {
        let workers = (0..thread::available_parallelism().unwrap().get())
            .map(|_i| {
                let (in_tx, in_rx) = mpsc::channel();
                let out_tx = out_tx.clone();
                (
                    in_tx,
                    s.spawn(move || {
                        while let Ok(lines) = in_rx.recv() {
                            let result = map_phase::<_, 50_000>(lines);
                            if result.is_empty() {
                                break;
                            }
                            out_tx.send(result).unwrap();
                        }
                    }),
                )
            })
            .collect::<Vec<_>>();
        drop(out_tx);

        let mut current_worker = 0;
        let mut lines = input.lines();
        loop {
            let work = lines.clone();
            workers[current_worker]
                .0
                .send(work.take(BATCH_SIZE))
                .unwrap();
            if lines.nth(BATCH_SIZE).is_some() {
                current_worker = (current_worker + 1) % workers.len();
            } else {
                break;
            }
        }

        for (i, _) in workers {
            i.send(lines.clone().take(0)).ok();
        }

        let mut result: HashMap<&str, (u16, F, F, F)> = HashMap::with_capacity(50_000);
        while let Ok(worker_result) = out_rx.recv() {
            for (location, (count, min, max, total)) in worker_result {
                result
                    .entry(location)
                    .and_modify(|(w_count, w_min, w_max, w_total)| {
                        *w_count += count;
                        if min < *w_min {
                            *w_min = min;
                        }
                        if max > *w_max {
                            *w_max = max;
                        }
                        *w_total += total;
                    })
                    .or_insert((count, min, max, total));
            }
        }

        result
            .into_iter()
            .map(|(location, (count, min, max, total))| {
                (location, (count, min, max, total / F::from(count)))
            })
    })
}
