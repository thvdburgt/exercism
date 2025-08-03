/// Returns the nth prime number, where n is 0-indexed.
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if primes.iter().all(|&i| candidate % i != 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
