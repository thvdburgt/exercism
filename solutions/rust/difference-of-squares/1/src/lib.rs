/// Square of sum of 1...n
pub fn square_of_sum(n: u32) -> u32 {
    (n * n * (n * n + 2 * n + 1)) / 4
}

/// Sum of squares of 1...n
pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

/// Difference between square of sum of 1...n and sum of squares of 1...n
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
