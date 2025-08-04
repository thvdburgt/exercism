/// Abbreviates a phrase into an acronym.
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .fold(
            (String::new(), true, false),
            |(mut acronym, start_new_word, prev_was_lowercase), c| {
                if (start_new_word && c.is_alphabetic()) || (prev_was_lowercase && c.is_uppercase())
                {
                    acronym.push(c.to_ascii_uppercase());
                }
                (acronym, (!c.is_alphabetic() && c != '\''), c.is_lowercase())
            },
        )
        .0
}
