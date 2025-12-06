use crate::types::DaySolver;

pub struct Solver;

#[derive(Clone, Debug)]
struct Problem {
    values: Vec<u64>,
    op: char,
}

pub struct Grid<'a> {
    values: Vec<Vec<&'a str>>,
    ops: Vec<char>,
}

fn parse_day_1(grid: &Grid) -> Vec<Problem> {
    grid.values
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let values: Vec<u64> = col
                .iter()
                .map(|&s| s.trim().parse::<u64>().unwrap())
                .collect();
            let op = grid.ops[i];
            Problem { values, op }
        })
        .collect()
}

fn parse_day_2(grid: &Grid, test: bool) -> Vec<Problem> {
    let mut problems = Vec::with_capacity(grid.values.len());
    for (i, col) in grid.values.iter().enumerate() {
        // the actual values are encoded by column in the values
        // so we have to gather the last character of each value, then the second last, etc.
        let mut max_len = 0;
        for &s in col {
            if s.len() > max_len {
                max_len = s.len();
            }
        }
        let mut values = vec![0; max_len];
        let mut exp = vec![0; max_len];
        for &s in col.iter().rev() {
            let bytes = s.as_bytes();
            for (i, &b) in bytes.iter().rev().enumerate() {
                if b == b' ' {
                    continue;
                }
                let digit = u64::from(b - b'0');
                values[i] += digit * 10u64.pow(exp[i]);
                exp[i] += 1;
            }
        }
        let op = grid.ops[i];
        problems.push(Problem { values, op });
    }
    if test {
        println!("Parsed problems for day 2:\n {problems:#?}");
    }

    problems
}

fn solve_problems(input: Vec<Problem>) -> String {
    let mut sum = 0;
    for problem in input {
        let result: u64 = match problem.op {
            '+' => problem.values.iter().sum(),
            '*' => problem.values.iter().product(),
            _ => panic!("Unknown operation: {}", problem.op),
        };
        sum += result;
    }
    sum.to_string()
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Grid<'a>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        // let mut ops = Vec::new();
        // let mut lines = input.lines();
        //
        // let first_line = lines.next().unwrap();
        // let first_parts: Vec<&str> = first_line.split_whitespace().collect();
        // let mut values = vec![Vec::new(); first_parts.len()];
        // for (i, val_str) in first_parts.iter().enumerate() {
        //     values[i].push(*val_str);
        // }
        // for line in lines {
        //     let parts: Vec<&str> = line.split_whitespace().collect();
        //     if parts.is_empty() {
        //         continue;
        //     }
        //     if parts[0].as_bytes()[0].is_ascii_digit() {
        //         for (i, val_str) in parts.iter().enumerate() {
        //             values[i].push(*val_str);
        //         }
        //     } else {
        //         ops = parts.iter().map(|&s| s.as_bytes()[0] as char).collect();
        //     }
        // }
        // if test {
        //     println!("Parsed grid:\n values: {values:#?}\n ops: {ops:#?}");
        // }

        // find where the columns are actually split.
        // number are not left or right aligned, so we have to check the positions of the operators
        // in the last line
        let mut ops = Vec::new();
        let mut col_indices = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        let last_line = lines[lines.len() - 1];
        for (i, c) in last_line.chars().enumerate() {
            if c != ' ' {
                ops.push(c);
                col_indices.push(i);
            }
        }

        let mut values = vec![Vec::new(); ops.len()];
        for line in &lines[0..lines.len() - 1] {
            if line.trim().is_empty() {
                continue;
            }
            for i in 0..col_indices.len() {
                let start = col_indices[i];
                let end = if i + 1 < col_indices.len() {
                    col_indices[i + 1] - 1
                } else {
                    line.len()
                };
                let val_str = &line[start..end];
                values[i].push(val_str);
            }
        }
        if test {
            println!("Parsed grid:\n values: {values:#?}\n ops: {ops:#?}");
        }
        Grid { values, ops }
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let input = parse_day_1(input);
        solve_problems(input)
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let input = parse_day_2(input, test);
        solve_problems(input)
    }
}
