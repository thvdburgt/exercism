use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

fn gather_letter_set(input: &str) -> BTreeSet<char> {
    let mut letters = BTreeSet::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            letters.insert(c);
        }
    }
    letters
}

fn calc_expression(input: &str) -> Option<isize> {
    let mut sum = 0;
    for addend in input.split('+').map(|s| s.trim()) {
        if addend.starts_with('0') && addend.len() > 1 {
            return None;
        }
        sum += addend.parse::<isize>().ok()?;
    }
    Some(sum)
}

fn check_equality(equation: &str) -> Option<bool> {
    let (lhs, rhs) = equation.split_once("==")?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();

    let lhs_value = calc_expression(lhs)?;
    let rhs_value = calc_expression(rhs)?;

    Some(lhs_value == rhs_value)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    println!("input: {}", input);
    // create set of all letters
    let letters = gather_letter_set(input);

    let nr_of_letters = letters.len();

    let permutations = (0u8..=9).permutations(nr_of_letters);

    for perm in permutations {
        assert_eq!(perm.len(), nr_of_letters);
        let mut output = input.to_owned();

        let mapping: HashMap<char, u8> = letters.iter().cloned().zip(perm).collect();
        for (&from, to) in &mapping {
            output = output.replace(from, &to.to_string());
        }
        if check_equality(&output) == Some(true) {
            println!("{}", output);
            return Some(mapping);
        }
    }

    None
}
