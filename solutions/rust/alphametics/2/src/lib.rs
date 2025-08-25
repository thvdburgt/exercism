use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// Represents a problem instance to be solved.
#[derive(Debug)]
struct Problem {
    // Each addend is a vector of chars, reversed (least significant digit first)
    summation_left: Vec<Vec<char>>,
    summation_right: Vec<Vec<char>>,
    max_nr_of_places: usize,
    must_be_nonzero: HashSet<char>,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Helper to convert a summation string into a vector of addends
        // Each addend is a vector of chars, reversed (least significant digit first)
        let to_addends = |summation_str: &str| -> Vec<Vec<char>> {
            summation_str
                .trim()
                .split('+')
                .map(|s| s.trim().chars().rev().collect())
                .collect()
        };

        let (lhs, rhs) = input.split_once("==").ok_or(())?;
        let summation_left = to_addends(lhs);
        let summation_right = to_addends(rhs);

        assert!(!summation_left.is_empty());
        assert!(!summation_right.is_empty());

        let max_nr_of_digits = summation_left
            .iter()
            .chain(summation_right.iter())
            .map(Vec::len)
            .max()
            .expect("Each summation must have at least one addend");

        // The leading digit of a multi-digit number must not be zero.
        let must_be_nonzero: HashSet<char> = summation_left
            .iter()
            .chain(summation_right.iter())
            .filter(|n| n.len() > 1)
            .map(|n| n.last().expect("All addends are non-empty"))
            .copied()
            .collect();

        Ok(Problem {
            summation_left,
            summation_right,
            max_nr_of_places: max_nr_of_digits,
            must_be_nonzero,
        })
    }
}

/// Represents a candidate solution during the search process.
#[derive(Debug)]
struct Candidate {
    mapping: HashMap<char, u8>,
    next_place_to_process: usize,
    carry_over_left: usize,
    carry_over_right: usize,
}

fn solve_step(problem: &Problem, partial: &Candidate) -> Vec<Candidate> {
    // Helper to determine the summation only for the current digit place
    let get_place_summation = |summation: &Vec<Vec<char>>| -> Vec<char> {
        summation
            .iter()
            .flat_map(|n| n.get(partial.next_place_to_process))
            .copied()
            .collect()
    };

    let place_summation_left = get_place_summation(&problem.summation_left);
    let place_summation_right = get_place_summation(&problem.summation_right);

    let unassigned_letters: HashSet<char> = place_summation_left
        .iter()
        .chain(place_summation_right.iter())
        .copied()
        .filter(|c| !partial.mapping.contains_key(c))
        .collect();

    let assigned_digits: HashSet<u8> = partial.mapping.values().copied().collect();
    let unassigned_digits: HashSet<u8> = (0..=9).filter(|d| !assigned_digits.contains(d)).collect();

    // Generate all possible permutations of the remaining unassigned digits
    // to assign to the undetermined letters
    let permutations = unassigned_digits
        .into_iter()
        .permutations(unassigned_letters.len());

    let mut new_partials = Vec::new();
    for perm in permutations {
        if unassigned_letters
            .iter()
            .zip(perm.iter())
            .any(|(c, d)| *d == 0 && problem.must_be_nonzero.contains(c))
        {
            // leading zero not allowed
            continue;
        }

        assert!(perm.len() == unassigned_letters.len());

        let mut mapping: HashMap<char, u8> = partial.mapping.clone();
        mapping.extend(unassigned_letters.iter().cloned().zip(perm));
        let mapping = mapping;

        let lhs_sum = partial.carry_over_left
            + place_summation_left
                .iter()
                .map(|c| {
                    *mapping
                        .get(c)
                        .expect("c must either be determine already, or be in perm")
                        as usize
                })
                .sum::<usize>();
        let rhs_sum = partial.carry_over_right
            + place_summation_right
                .iter()
                .map(|c| {
                    *mapping
                        .get(c)
                        .expect("c must either be determine already, or be in perm")
                        as usize
                })
                .sum::<usize>();

        if lhs_sum % 10 == rhs_sum % 10 {
            let carry_over_left = lhs_sum / 10;
            let carry_over_right = rhs_sum / 10;

            let new_partial = Candidate {
                mapping,
                next_place_to_process: partial.next_place_to_process + 1,
                carry_over_left,
                carry_over_right,
            };

            new_partials.push(new_partial);
        }
    }

    new_partials
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let problem = Problem::from_str(input).expect("The given input should always be valid");

    let initial_candidate = Candidate {
        mapping: HashMap::new(),
        next_place_to_process: 0,
        carry_over_left: 0,
        carry_over_right: 0,
    };

    let mut candidates = vec![initial_candidate];
    for _ in 0..problem.max_nr_of_places {
        candidates = candidates
            .into_iter()
            .flat_map(|partial| solve_step(&problem, &partial))
            .collect();
    }

    if candidates.is_empty() {
        None
    } else {
        assert!(candidates.len() == 1);
        Some(candidates.pop().expect("len == 1").mapping)
    }
}
