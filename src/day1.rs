use std::str::FromStr;

use aoc::Solver;

pub struct Aoc;

impl Solver for Aoc {
    // Change the type of Item to the one that will be processed by the solver
    type Item = String;

    fn puzzle() -> aoc::Puzzle {
        aoc::Puzzle::new(2023, 1)
    }

    fn solve_one(inputs: &[Self::Item]) -> String {
        let sum: usize = inputs
            .iter()
            .map(|line| {
                line.as_str()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    //.map(|c| usize::from_str(c.to_string().as_str()).unwrap())
                    .collect()
            })
            .map(|digits: Vec<char>| {
                format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            })
            .map(|s: String| usize::from_str(&s).unwrap())
            .sum();

        sum.to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        "todo".to_string()
    }
}

fn find_num(line: &str) -> Vec<usize> {
    let mut found: Vec<(usize, usize)> = Vec::with_capacity(8);
    if let Some(i) = line.find("one") {
        found.push((i, 1))
    };
}
