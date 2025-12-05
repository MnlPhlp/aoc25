use std::{collections::HashSet, ops::RangeInclusive};

use crate::types::DaySolver;

pub struct Solver;

pub struct Database {
    fresh: Vec<RangeInclusive<u64>>,
    available: Vec<u64>,
}
impl<'a> DaySolver<'a> for Solver {
    type Input = Database;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        let mut fresh = Vec::new();
        let mut available = Vec::new();
        let mut lines = input.lines();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            let (start, end) = line.trim().split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            fresh.push(RangeInclusive::new(start, end));
        }
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let id: u64 = line.trim().parse().unwrap();
            available.push(id);
        }
        // merge fresh ranges to non overlapping ranges
        let mut combined_ranges: Vec<RangeInclusive<u64>> = Vec::new();
        while combined_ranges.len() < fresh.len() {
            if !combined_ranges.is_empty() {
                std::mem::swap(&mut fresh, &mut combined_ranges);
            }
            combined_ranges.clear();
            for range in &fresh {
                let overlapping = combined_ranges
                    .iter()
                    .position(|r| *range.start() <= *r.end() + 1 && *range.end() >= *r.start() - 1);
                if let Some(index) = overlapping {
                    let existing_range = &combined_ranges[index];
                    let new_start = *existing_range.start().min(range.start());
                    let new_end = *existing_range.end().max(range.end());
                    combined_ranges[index] = RangeInclusive::new(new_start, new_end);
                } else {
                    combined_ranges.push(range.clone());
                }
            }
        }
        Database {
            fresh: combined_ranges,
            available,
        }
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut fresh = 0;
        for id in &input.available {
            if input.fresh.iter().any(|range| range.contains(id)) {
                fresh += 1;
            }
        }
        fresh.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut total_fresh = 0;
        // ranges are already merged in parse_input
        for range in &input.fresh {
            total_fresh += range.end() - range.start() + 1;
        }
        total_fresh.to_string()
    }
}
