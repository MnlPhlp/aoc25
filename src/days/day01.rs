use crate::types::DaySolver;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut pos = 50;
        let mut zeroes = 0;
        for line in input.lines() {
            let (dir, value) = line.split_at(1);
            let value: i32 = value.parse::<i32>().unwrap() % 100;
            match dir {
                "L" => pos = (pos - value + 100) % 100,
                "R" => pos = (pos + value) % 100,
                _ => unreachable!(),
            }
            if pos == 0 {
                zeroes += 1;
            }
        }
        format!("{zeroes}")
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut pos = 50;
        let mut zeroes = 0;
        for line in input.lines() {
            let (dir, value) = line.split_at(1);
            let value: i32 = value.parse().unwrap();
            // count full cycles that cross zero
            let div = value / 100;
            let rem = value % 100;
            zeroes += div;
            // update pos and check if we cross zero in the remainder
            match dir {
                "L" => {
                    let new_pos = (pos - rem + 100) % 100;
                    if (new_pos > pos && pos != 0) || new_pos == 0 {
                        zeroes += 1;
                    }
                    pos = new_pos;
                }
                "R" => {
                    let new_pos = (pos + rem) % 100;
                    if new_pos < pos {
                        zeroes += 1;
                    }
                    pos = new_pos;
                }
                _ => unreachable!(),
            }
            if test {
                println!("Line: {line}, Pos: {pos}, Zeroes: {zeroes}");
            }
        }
        format!("{zeroes}")
    }
}
