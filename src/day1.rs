use aoc::Solver;

pub struct Aoc;

impl Solver for Aoc {
    // Change the type of Item to the one that will be processed by the solver
    type Item = String;

    fn solve_one(_inputs: &[Self::Item]) -> String {
        "This is the answer of the first part of the puzzle".to_string()
    }

    // Remove these commments when solving part 2!
    // fn solve_two(inputs: &[Self::Item]) -> String {
    //     "This is the answer of the second part of the puzzle".to_string()
    // }
}
