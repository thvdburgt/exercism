/// Returns the nth prime number, where n is 0-indexed.
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    for candidate in 2..=u32::MAX {
        // Check if candidate is prime by checking if no n <= (sqrt(candidate) + 1) divides it.
        if !primes
            .iter()
            .take_while(|&p| *p <= (f64::from(candidate)).sqrt() as u32 + 1)
            .any(|p| candidate % p == 0)
        {
            // Cadidate is a prime, check if it is the nth prime.
            if primes.len() == n as usize {
                return candidate;
            } else {
                primes.push(candidate)
            }
        }
    }

    0 // This line should never be reached, but is required to satisfy the return type.
}
