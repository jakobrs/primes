use std::time::Instant;

// From jakobrs/primes, using the kimwalisch/primesieve library
use primesieve::PrimesieveIterator;

fn main() {
    println!("  n |      t_n |      i+1 | t_n*n/e^n | (i+1)n/e^n |   p_{{i+1}} | p_{{i+1}}/e^n |         time");
    println!("-------------------------------------------------------------------------------------------");
    for n in 1.. {
        let count_f64 = ((n as f64 + 1.5).exp() / n as f64).ceil();
        let count = count_f64 as u64;

        let before = Instant::now();

        let results = PrimesieveIterator::new_start_stop(1, (count_f64 * count_f64.ln()) as u64)
            .enumerate()
            .skip(2)
            .take(count as usize - 1)
            .map(|(i, p)| {
                let i = i as i64;
                let p = p as i64;
                let res = n * i + 3 - p;

                (res, i, p)
            })
            .maxes();

        for (t, i, p) in results {
            println!(
                " {n: >2} | {t: >8} | {i_prime: >8} | {: >9.3} | {: >10.3} | {p: >9} | {: >11.3} | {elapsed: >12?}",
                t as f64 * n as f64 / (n as f64).exp(),
                (i as f64 + 1.) * (n as f64) / (n as f64).exp(),
                p as f64 / (n as f64).exp(),
                i_prime = i + 1,
                elapsed = before.elapsed(),
            );
        }
    }
}

trait IteratorExt: Iterator<Item = (i64, i64, i64)> {
    fn maxes(self) -> Vec<(i64, i64, i64)>;
}

impl<I: Iterator<Item = (i64, i64, i64)>> IteratorExt for I {
    fn maxes(self) -> Vec<(i64, i64, i64)> {
        let mut res = 0;
        let mut equivalents = vec![];

        for (t, i, p) in self {
            if t > res {
                equivalents.clear();
                res = t;
            }
            if t == res {
                equivalents.push((t, i, p));
            }
        }

        equivalents
    }
}
