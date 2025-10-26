use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut encountered_chars = HashSet::new();

    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| encountered_chars.insert(c))
}
