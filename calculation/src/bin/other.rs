use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    const M: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

    let iter = (1..3)
        .cartesian_product(1..5)
        .cartesian_product(1..7)
        .cartesian_product(1..11)
        .cartesian_product(1..13)
        .cartesian_product(1..17)
        .cartesian_product(1..19)
        .cartesian_product(1..23)
        .par_bridge()
        .map(|(((((((r3, r5), r7), r11), r13), r17), r19), r23)| {
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

    let min_b = ParallelIterator::min(iter);

    println!("Min B: {:?}", min_b);
}
