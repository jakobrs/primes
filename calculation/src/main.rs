use primesieve::PrimesieveIterator;

fn main() {
    println!("Hello, world!");

    let max_q = PrimesieveIterator::new_start_stop(0, 223092870)
        .take_while(|&p| p <= 223092870)
        .enumerate()
        .map(|(i, prime)| {
            let i = i as i64;
            let prime = prime as i64;

            6 * i - prime + 3
        })
        .max();

    println!("Max Q: {:?}", max_q);
}
