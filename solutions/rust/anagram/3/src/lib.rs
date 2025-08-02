use std::collections::{HashMap, HashSet};

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();

    if s1 == s2 {
        return false;
    }

    let mut s1_chars = HashMap::new();
    s1.chars().for_each(|c| {
        s1_chars
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let mut s2_chars = HashMap::new();
    s2.chars().for_each(|c| {
        s2_chars
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    dbg!(s1_chars) == dbg!(s2_chars)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .into_iter()
        .cloned()
        .filter(|&x| are_anagrams(word, x))
        .collect()
}
