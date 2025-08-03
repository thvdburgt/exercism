/// Given a string of digits, output all the contiguous substrings of a given length.
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return Vec::new();
    }

    (0..=(digits.len() - len))
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
