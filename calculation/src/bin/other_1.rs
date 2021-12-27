use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    // ...

    const M: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

    let iter = [3, 5, 7, 11, 13, 17, 19, 23]
        .into_iter()
        .map(|n| 1..n)
        .multi_cartesian_product()
        .par_bridge()
        .map(|r| {
            let r3 = r[0];
            let r5 = r[1];
            let r7 = r[2];
            let r11 = r[3];
            let r13 = r[4];
            let r17 = r[5];
            let r19 = r[6];
            let r23 = r[7];

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
        })
        .min();

    println!("Min B: {:?}", iter);
}
