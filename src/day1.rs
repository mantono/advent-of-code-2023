use std::{collections::BTreeMap, str::FromStr};

use aoc::Solver;

pub struct Aoc;

impl Solver for Aoc {
    type Item = String;

    fn puzzle() -> aoc::Puzzle {
        aoc::Puzzle::new(2023, 1)
    }

    fn solve_one(inputs: &[Self::Item]) -> String {
        let sum: u32 = inputs
            .iter()
            .map(|line| {
                line.as_str()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| u32::from_str(c.to_string().as_str()).unwrap())
                    .collect()
            })
            .map(|digits: Vec<u32>| concat_num(*digits.first().unwrap(), *digits.last().unwrap()))
            .sum();

        sum.to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        let sum: u32 = inputs
            .iter()
            .map(|line| find_num(line))
            .map(|tree| {
                (
                    tree.first_key_value().unwrap().1.clone(),
                    tree.last_key_value().unwrap().1.clone(),
                )
            })
            .map(|(left, right)| concat_num(left, right))
            .sum();
        sum.to_string()
    }
}

fn concat_num(left: u32, right: u32) -> u32 {
    (left * 10) + right
}

fn find_num(line: &str) -> BTreeMap<usize, u32> {
    let mut found: BTreeMap<usize, u32> = BTreeMap::new();
    for (i, c) in line.char_indices() {
        let digit: Option<u32> = if let Some(d) = c.to_digit(10) {
            Some(d)
        } else if let Some(letters) = line.get(i..) {
            if letters.starts_with("one") {
                Some(1)
            } else if letters.starts_with("two") {
                Some(2)
            } else if letters.starts_with("three") {
                Some(3)
            } else if letters.starts_with("four") {
                Some(4)
            } else if letters.starts_with("five") {
                Some(5)
            } else if letters.starts_with("six") {
                Some(6)
            } else if letters.starts_with("seven") {
                Some(7)
            } else if letters.starts_with("eight") {
                Some(8)
            } else if letters.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(digit) = digit {
            found.insert(i, digit);
        }
    }
    found
}
