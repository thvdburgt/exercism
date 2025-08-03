/// Sums the multiples of the given factors that are less than the specified limit.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.iter().any(|&f| f != 0 && n % f == 0))
        .sum()
}
