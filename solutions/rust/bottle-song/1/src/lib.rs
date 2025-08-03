pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|n| verse(start_bottles - n))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn to_word(n: u32) -> &'static str {
    assert!(n > 0);
    assert!(n <= 10);
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => unreachable!(),
    }
}

fn verse(n: u32) -> String {
    assert!(n > 0);
    assert!(n <= 10);

    let first_line = match n {
        1 => "One green bottle hanging on the wall,".into(),
        _ => format!("{0} green bottles hanging on the wall,", to_word(n)),
    };
    let last_line = match n - 1 {
        0 => "There'll be no green bottles hanging on the wall.".into(),
        1 => "There'll be one green bottle hanging on the wall.".into(),
        _ => format!(
            "There'll be {0} green bottles hanging on the wall.",
            to_word(n - 1).to_lowercase()
        ),
    };

    format!(
        "{first_line}\n{first_line}\nAnd if one green bottle should accidentally fall,\n{last_line}"
    )
}
