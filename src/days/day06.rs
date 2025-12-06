use crate::types::DaySolver;

pub struct Solver;

pub struct Problems {
    values: Vec<Vec<u64>>,
    ops: Vec<u8>,
}

fn parse_day_1(input: &str) -> Problems {
    // TODO: maybe walk bytes and do splitting manually
    let lines = input.lines().collect::<Vec<_>>();
    let mut numbers = lines[0]
        .split(' ')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| vec![x.parse::<u64>().unwrap()])
        .collect::<Vec<Vec<_>>>();
    let mut ops = vec![0u8; numbers.len()];
    for l in 1..lines.len() {
        let parts = lines[l].split(' ').map(str::trim).filter(|x| !x.is_empty());
        for (i, part) in parts.enumerate() {
            if l == lines.len() - 1 {
                // ops line
                ops[i] = part.as_bytes()[0];
            } else {
                let value = part.parse::<u64>().unwrap();
                numbers[i].push(value);
            }
        }
    }
    Problems {
        values: numbers,
        ops,
    }
}

fn solve_problems(input: Problems) -> String {
    let mut sum = 0;
    for (values, op) in input.values.iter().zip(input.ops.iter()) {
        let result: u64 = match *op as char {
            '+' => values.iter().sum(),
            '*' => values.iter().product(),
            _ => panic!("Unknown operation: {}", *op as char),
        };
        sum += result;
    }
    sum.to_string()
}

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let input = parse_day_1(input);
        solve_problems(input)
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
