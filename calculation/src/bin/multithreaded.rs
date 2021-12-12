use std::time::Instant;

use primesieve::PrimesieveIterator;

fn main() {
    const DIST: u64 = 223092870;
    const COEFFICIENT: i64 = 6;

    let n_threads = num_cpus::get() as u64;
    let thread_dist = DIST / n_threads + 1;

    let start_time = Instant::now();
    let res = (0..n_threads)
        .map(|thread| {
            std::thread::spawn(move || {
                let start = thread * thread_dist;
                let stop = u64::min(start + thread_dist, DIST);

                let mut i = 0;

                let max_q = PrimesieveIterator::new_start_stop(start, stop)
                    .take_while(|&p| p <= stop)
                    .map(|prime| {
                        let prime = prime as i64;

                        let q = COEFFICIENT * i - prime + 3;
                        i += 1;

                        q
                    })
                    .max()
                    .unwrap();

                (i, max_q)
            })
        })
        .collect::<Vec<_>>();

    let mut acc_i = 0;
    let mut total_max_q = i64::MIN;
    for thread in res {
        let (i, mut max_q) = thread.join().unwrap();

        // The threads don't know the number of primes less than `start`, so instead of initialising `i` to `pi(start)`, we just add `COEFFICIENT*pi(start)` here.
        max_q += COEFFICIENT * acc_i;
        acc_i += i;

        if max_q > total_max_q {
            total_max_q = max_q;
        }
    }

    println!("Time elapsed: {:?}", start_time.elapsed());
    println!("Max Q: {}", total_max_q);
    println!("Primes counted: {}", acc_i);
}
