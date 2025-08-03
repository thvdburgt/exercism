/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        // Start at the end of the code.
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10)
                // Double every second digit.
                .map(|n| if count % 2 == 1 { n * 2 } else { n })
                // If the result of doubling a digit is greater than 9, we subtract 9 from it.
                .map(|n| if n > 9 { n - 9 } else { n })
                .map(|n| (sum + n, count + 1))
        })
        // If the sum of all digits is divisible by 10, the code is valid.
        .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
}
