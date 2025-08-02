use std::collections::HashSet;

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();

    if s1 == s2 {
        false // Strings are not an anagram of themselves
    } else {
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();
        s1.sort();
        s2.sort();

        s1 == s2
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .into_iter()
        .cloned()
        .filter(|&x| are_anagrams(word, x))
        .collect()
}
