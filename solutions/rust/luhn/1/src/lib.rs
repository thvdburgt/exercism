/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Check that there are only digits and spaces in the string.
    if !code.chars().all(|c| c.is_ascii_digit() || c == ' ') {
        return false;
    }

    // Convert the string to a vec of numbers (the digits).
    let digits: Vec<_> = code.chars().filter_map(|c| c.to_digit(10)).collect();

    // Check that we have at least two digits.
    if digits.len() < 2 {
        return false;
    }

    digits
        .into_iter()
        // Double every second digit, starting from the end.
        .rev()
        .enumerate()
        .map(|(i, d)| if i % 2 == 0 { d } else { 2 * d })
        // If the result of doubling a digit is greater than 9, we subtract 9 from that result.
        .map(|d| if d > 9 { d - 9 } else { d })
        // Sum them and check if the sum is evenly divisible by 10.
        .sum::<u32>()
        % 10
        == 0
}
