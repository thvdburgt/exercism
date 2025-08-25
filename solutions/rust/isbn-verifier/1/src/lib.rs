/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // Filter out any dashes
    let chars: Vec<_> = isbn.chars().filter(|&c| c != '-').collect();

    // Ensure the cleaned ISBN has exactly 10 characters
    // and that all characters except possibly the last are digits,
    // with the last character allowed to be a digit or 'X'
    if chars.len() == 10
        && chars[..9].iter().all(|&c| c.is_ascii_digit())
        && (chars[9].is_ascii_digit() || chars[9] == 'X')
    {
        // Map all chars to digits, zip it with the integer weights, compute checksum.
        chars
            .iter()
            .map(|&c| {
                if c == 'X' {
                    10
                } else {
                    c.to_digit(10).expect("digit expected")
                }
            })
            .zip((1..=10).rev())
            .fold(0, |acc, (d, i)| acc + d * i)
            % 11
            == 0
    } else {
        false
    }
}
