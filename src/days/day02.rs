#![allow(clippy::cast_possible_truncation)]
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
                invalid_ids_sum += left_min * 10u64.pow(half_l as u32) + left_min;
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
                invalid_ids_sum += left_num * 10u64.pow(half_l as u32) + left_num;
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
                invalid_ids_sum += left_num * 10u64.pow(half_l as u32) + left_num;
            }
            left_max -= 1;
        }
        for left in left_min..=left_max {
            if left <= right_max && left >= right_min {
                invalid_ids_sum += left * 10u64.pow(half_l as u32) + left;
            }
        }
    }
    invalid_ids_sum
}

fn get_invalid_ids_sum_2(range: RangeInclusive<u64>) -> u64 {
    let mut invalid_ids = Vec::new();
    let start = range.start().to_string();
    let end = range.end().to_string();

    // check if there is a fixed prefix we have to include in the repeated part
    let fixed_prefix = if start.len() < end.len() {
        ""
    } else {
        let prefix_len = start
            .chars()
            .zip(end.chars())
            .take_while(|(a, b)| a == b)
            .count();
        &start[..prefix_len]
    };
    let prefix_len = fixed_prefix.len();

    // generate possible solutions
    for l in 1..=end.len() / 2 {
        // start and end with the fixed prefix
        let min = if prefix_len >= l {
            fixed_prefix[..l].parse::<u64>().unwrap()
        } else if prefix_len > 0 {
            fixed_prefix.parse::<u64>().unwrap() * 10u64.pow((l - prefix_len) as u32)
        } else {
            10u64.pow((l - 1) as u32)
        };
        let max = if prefix_len >= l {
            fixed_prefix[..l].parse::<u64>().unwrap()
        } else if prefix_len > 0 {
            fixed_prefix.parse::<u64>().unwrap() * 10u64.pow((l - prefix_len + 1) as u32) - 1
        } else {
            10u64.pow(l as u32) - 1
        };
        // generate and check repeated numbers
        for part in min..=max {
            let mut repeated = part;
            while repeated < *range.end() {
                repeated = repeated * 10u64.pow(l as u32) + part;
                if repeated >= *range.start()
                    && repeated <= *range.end()
                    && !invalid_ids.contains(&repeated)
                {
                    invalid_ids.push(repeated);
                    break;
                }
            }
        }
    }
    invalid_ids.iter().sum()
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

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut sum = 0;
        for range in input {
            sum += get_invalid_ids_sum_1(range.clone());
        }
        sum.to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let mut sum = 0;
        for range in input {
            sum += get_invalid_ids_sum_2(range.clone());
        }
        sum.to_string()
    }
}
