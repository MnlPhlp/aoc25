use crate::types::DaySolver;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
    }

    fn solve1(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
