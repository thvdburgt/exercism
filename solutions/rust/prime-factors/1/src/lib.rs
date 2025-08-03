/// Calculate the prime factors of the given number.
pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();

    let mut n = n;
    let mut candidate = 2;
    while n > 1 {
        if n % candidate == 0 {
            prime_factors.push(candidate);
            n /= candidate
        } else {
            candidate += 1;
        }
    }

    prime_factors
}
