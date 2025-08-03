pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.is_empty() {
        return proverb;
    }

    list.windows(2).for_each(|w| {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", w[0], w[1]));
    });

    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
