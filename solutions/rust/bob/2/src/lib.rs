pub fn reply(message: &str) -> &str {
    let is_yelling = message
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

    match message.trim() {
        m if m.trim().is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        _ if is_yelling => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
