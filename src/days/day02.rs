use std::ops::RangeInclusive;

use crate::types::DaySolver;

pub struct Solver;

fn get_invalid_ids_sum_1(range: RangeInclusive<u64>) -> u64 {
    let mut invalid_ids_sum = 0;
    let start = range.start().to_string();
    let end = range.end().to_string();
    for l in start.len()..=end.len() {
        // total repeat is only possible with even length
        if l % 2 != 0 {
            continue;
        }
        let half_l = l / 2;

        // calculate min and max for left and right parts
        let mut left_min = if l == start.len() {
            start[..(l / 2)].parse::<u64>().unwrap()
        } else {
            10u64.pow(half_l as u32 - 1)
        };
        let right_min = 10u64.pow(half_l as u32 - 1);
        let mut left_max = if l == end.len() {
            end[..half_l].parse::<u64>().unwrap()
        } else {
            10u64.pow(half_l as u32) - 1
        };
        let right_max = 10u64.pow(half_l as u32) - 1;

        if start.len() == end.len() && left_min == left_max {
            // left part is fixed, only need to check the right part range
            let right_min = start[half_l..].parse::<u64>().unwrap();
            let right_max = end[half_l..].parse::<u64>().unwrap();
            if left_min <= right_max && left_max >= right_min {
                invalid_ids_sum += (left_min * 10u64.pow(half_l as u32) + left_min);
            }
            continue;
        }

        // get numbers that are possible for both left and right parts
        // special case for the first left number because the right part did not wrap around yet
        // so assuming 1000... as min for right part is wrong
        if l == start.len() {
            let left_part = &start[..half_l];
            let right_part = &start[half_l..];
            let left_num = left_part.parse::<u64>().unwrap();
            let right_num = right_part.parse::<u64>().unwrap();
            if left_num >= right_num {
                invalid_ids_sum += (left_num * 10u64.pow(half_l as u32) + left_num);
            }
            left_min += 1;
        }
        // special case for the last left number because the right part will not wrap around
        // so assuming 9999... as max for right part is wrong
        if l == end.len() {
            let left_part = &end[..half_l];
            let right_part = &end[half_l..];
            let left_num = left_part.parse::<u64>().unwrap();
            let right_num = right_part.parse::<u64>().unwrap();
            if left_num <= right_num {
                invalid_ids_sum += (left_num * 10u64.pow(half_l as u32) + left_num);
            }
            left_max -= 1;
        }
        for left in left_min..=left_max {
            if left <= right_max && left >= right_min {
                invalid_ids_sum += (left * 10u64.pow(half_l as u32) + left);
            }
        }
    }
    invalid_ids_sum
}

fn get_invalid_ids_sum_2(range: RangeInclusive<u64>) -> u64 {
    let mut invalid_ids_sum = 0;
    for num in range {
        let num_str = num.to_string();
        for sub_l in 1..=(num_str.len() / 2) {
            if num_str.len() % sub_l != 0 {
                continue;
            }
            let mut is_invalid = true;
            let first_sub = &num_str[0..sub_l];
            for start in (sub_l..num_str.len()).step_by(sub_l) {
                let sub = &num_str[start..start + sub_l];
                if sub != first_sub {
                    is_invalid = false;
                    break;
                }
            }
            if is_invalid {
                invalid_ids_sum += num;
                break;
            }
        }
    }
    invalid_ids_sum
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<RangeInclusive<u64>>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let ranges = input
            .trim()
            .split(',')
            .map(|part| {
                let (start, end) = part.split_once('-').unwrap();
                let start = start.parse().unwrap();
                let end = end.parse().unwrap();
                RangeInclusive::new(start, end)
            })
            .collect();
        if test {
            println!("Parsed ranges: {ranges:?}");
        }
        ranges
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for range in input {
            sum += get_invalid_ids_sum_1(range.clone());
        }
        sum.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for range in input {
            sum += get_invalid_ids_sum_2(range.clone());
        }
        sum.to_string()
    }
}
