use crate::types::DaySolver;

pub struct Point {
    x: u64,
    y: u64,
}

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Point>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect()
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut max_size = 0u64;
        for (i, one) in input.iter().enumerate() {
            for two in &input[(i + 1)..] {
                let size = (one.x.abs_diff(two.x) + 1) * (one.y.abs_diff(two.y) + 1);
                if size > max_size {
                    max_size = size;
                }
            }
        }
        max_size.to_string()
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
