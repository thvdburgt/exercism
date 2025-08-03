/// Returns the nth prime number, where n is 0-indexed.
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut candidate = 2;
    while primes.len() <= n as usize {
        if primes
            .iter()
            .take_while(|&&p| p * p <= candidate)
            .all(|&p| candidate % p != 0)
        {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes[n as usize]
}
