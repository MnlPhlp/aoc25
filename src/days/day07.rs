use crate::types::DaySolver;

pub struct Input<'a> {
    start_col: usize,
    width: usize,
    rest: &'a str,
}

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Input<'a>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        let (first_line, rest) = input.split_once('\n').unwrap_or((input, ""));
        let width = first_line.len();
        let start_col = first_line.find('S').unwrap_or(width);
        Input {
            start_col,
            width,
            rest,
        }
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut splits = 0;
        let mut active = vec![false; input.width];
        active[input.start_col] = true;
        for line in input.rest.lines() {
            for (i, &c) in line.as_bytes().iter().enumerate() {
                if c == b'^' && active[i] {
                    splits += 1;
                    active[i] = false;
                    if i > 0 {
                        active[i - 1] = true;
                    }
                    if i + 1 < input.width {
                        active[i + 1] = true;
                    }
                }
            }
        }

        splits.to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let mut paths = 0;
        let mut active = vec![false; input.width];
        active[input.start_col] = true;
        for line in input.rest.lines() {
            for (i, &c) in line.as_bytes().iter().enumerate() {
                if c == b'^' && active[i] {
                    paths += 1;
                    active[i] = false;
                    if i > 0 {
                        active[i - 1] = true;
                    }
                    if i + 1 < input.width {
                        active[i + 1] = true;
                    }
                }
            }
        }

        paths.to_string()
    }
}
