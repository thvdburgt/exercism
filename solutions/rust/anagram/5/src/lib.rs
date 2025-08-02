use std::collections::{HashMap, HashSet};

fn are_anagrams(s1: &str, s2: &str) -> bool {
    fn lowercase_chars(s: &str) -> impl Iterator<Item = char> {
        s.chars().flat_map(char::to_lowercase)
    }

    if lowercase_chars(s1).eq(lowercase_chars(s2)) {
        return false;
    }
    
    fn count_chars(s: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        lowercase_chars(s).for_each(|c| {
            *counts.entry(c).or_insert(0) += 1;
        });
        counts
    }

    count_chars(s1) == count_chars(s2)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter().copied()
        .filter(|&x| are_anagrams(word, x))
        .collect()
}
