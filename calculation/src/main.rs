use primesieve::PrimesieveIterator;

fn main() {
    println!("Hello, world!");

    let mut max_q = i64::MIN;

    for (i, prime) in PrimesieveIterator::new_start_stop(0, 223092870)
        .take_while(|&p| p <= 223092870)
        .enumerate()
    {
        let i = i as i64;
        let prime = prime as i64;

        let q = 6 * i - prime + 3;
        if q > max_q {
            max_q = q;
        }
    }

    println!("Max Q: {}", max_q);
}
