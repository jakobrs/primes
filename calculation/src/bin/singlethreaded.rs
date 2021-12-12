use primesieve::PrimesieveIterator;

fn main() {
    const DIST: u64 = 223_092_870;
    const COEFFICIENT: i64 = 7;

    let max_q = PrimesieveIterator::new_start_stop(0, DIST)
        .take_while(|&p| p <= DIST)
        .enumerate()
        .map(|(i, prime)| {
            let i = i as i64;
            let prime = prime as i64;

            COEFFICIENT * i - prime + 3
        })
        .max();

    println!("Max Q: {:?}", max_q);
}
