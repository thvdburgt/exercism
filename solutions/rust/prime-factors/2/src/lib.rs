/// Calculate the prime factors of the given number.
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut candidate = 2;

    while candidate * candidate <= n {
        while n % candidate == 0 {
            prime_factors.push(candidate);
            n /= candidate;
        }
        candidate += if candidate == 2 { 1 } else { 2 }; // After 2, check only odd numbers
    }

    if n > 1 {
        prime_factors.push(n);
    }

    prime_factors
}
