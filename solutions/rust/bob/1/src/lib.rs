pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.trim().ends_with('?');
    let is_shouting = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .try_fold(0, |count, c| {
            if c.is_uppercase() {
                Some(count + 1)
            } else {
                None
            }
        })
        .is_some_and(|count| count > 0);

    if is_question {
        if is_shouting {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_shouting {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
