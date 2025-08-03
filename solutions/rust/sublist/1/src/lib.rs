#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(l1: &[i32], l2: &[i32]) -> Comparison {
    if l1 == l2 {
        Comparison::Equal
    } else if l1.is_empty() || l2.windows(l1.len()).any(|l| l == l1) {
        Comparison::Sublist
    } else if l2.is_empty() || l1.windows(l2.len()).any(|l| l == l2) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
