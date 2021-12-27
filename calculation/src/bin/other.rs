use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use cpu_time::ProcessTime;
use itertools::Itertools;

fn main() {
    const M: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

    let count_iterations = Arc::new(AtomicUsize::new(0));

    {
        let count_iterations = count_iterations.clone();

        std::thread::spawn(move || loop {
            let before = Instant::now();
            let before_cpu = ProcessTime::now();
            std::thread::sleep(std::time::Duration::from_secs(10));
            let count = count_iterations.swap(0, Ordering::Relaxed) as f32;
            println!(
                "Iterations per second:     {}",
                count / before.elapsed().as_secs_f32()
            );
            println!(
                "Iterations per CPU second: {}",
                count / before_cpu.elapsed().as_secs_f32()
            );
        });
    }

    let mut handles = vec![];
    for r3 in 1..3 {
        for r5 in 1..5 {
            let count_iterations = count_iterations.clone();
            handles.push(std::thread::spawn(move || {
                let iter = (1..7)
                    .cartesian_product(1..11)
                    .cartesian_product(1..13)
                    .cartesian_product(1..17)
                    .cartesian_product(1..19)
                    .cartesian_product(1..23)
                    .map(|(((((r7, r11), r13), r17), r19), r23)| {
                        count_iterations.fetch_add(1, Ordering::SeqCst);
                        //eprintln!("a");

                        let mut count = 1;

                        (2..M + 1)
                            .step_by(2)
                            .filter_map(|a| {
                                if (r3 + a) % 3 == 0
                                    && (r5 + a) % 5 == 0
                                    && (r7 + a) % 7 == 0
                                    && (r11 + a) % 11 == 0
                                    && (r13 + a) % 13 == 0
                                    && (r17 + a) % 17 == 0
                                    && (r19 + a) % 19 == 0
                                    && (r23 + a) % 23 == 0
                                {
                                    count += 1;

                                    Some(a - 6 * count)
                                } else {
                                    None
                                }
                            })
                            .min()
                    });
                Iterator::min(iter)
            }));
            handles.pop().unwrap().join().unwrap(); // Comment to evaluate in parallel
        }
    }

    let min_b = handles
        .into_iter()
        .map(std::thread::JoinHandle::join)
        .map(Result::unwrap)
        .min();

    println!("Min B: {:?}", min_b);
}
