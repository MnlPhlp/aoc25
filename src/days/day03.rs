use crate::types::DaySolver;

pub struct Solver;

fn get_max_digit(input: &str) -> (u32, usize) {
    let mut max_val = 0;
    let mut max_pos = 0;
    for (i, ch) in input.as_bytes().iter().enumerate() {
        let val = u32::from(ch - b'0');
        if val > max_val {
            max_val = val;
            max_pos = i;
        }
    }
    (max_val, max_pos)
}

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for bank in input.lines() {
            if test {
                println!("Bank: {bank}");
            }
            let (max_val, max_pos) = get_max_digit(&bank[..bank.len() - 1]);
            let (second_val, _) = get_max_digit(&bank[max_pos + 1..]);
            let val = max_val * 10 + second_val;
            if test {
                println!("Max: {max_val} at {max_pos}, Second: {second_val}, Value: {val}");
            }
            sum += val;
        }
        sum.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for bank in input.lines() {
            if test {
                println!("Bank: {bank}");
            }
            let mut num = 0;
            let mut last_max_pos = 0;
            for i in 0..12 {
                let (max_val, max_pos) = get_max_digit(&bank[last_max_pos..bank.len() - (11 - i)]);
                num += u64::from(max_val) * 10u64.pow(11 - i as u32);
                last_max_pos += max_pos + 1;
            }
            if test {
                println!("Number: {num}");
            }
            sum += num;
        }
        sum.to_string()
    }
}
