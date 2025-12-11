use std::collections::{HashSet, VecDeque};

use crate::types::DaySolver;

#[derive(Debug)]
pub struct MachineData {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<MachineData>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let machines = input
            .lines()
            .map(|line| {
                let lights_end = line.find(']').unwrap();
                let on_required = line[1..lights_end]
                    .bytes()
                    .enumerate()
                    .filter_map(|(i, b)| if b == b'#' { Some(i as u8) } else { None });
                // turn into bit flags
                let target = on_required.map(|i| 1 << i).sum();

                let joltage_start = line[lights_end..].find('{').unwrap() + lights_end;
                let buttons = &line[lights_end + 2..joltage_start - 1];
                let buttons = buttons
                    .split(' ')
                    .map(|btn| {
                        let values = btn[1..btn.len() - 1].split(',').map(|s| s.parse().unwrap());
                        // turn into bit flags
                        values.map(|i: u8| 1 << i).sum()
                    })
                    .collect();

                let joltages = &line[joltage_start + 1..line.len() - 1];
                let joltages = joltages.split(',').map(|s| s.parse().unwrap()).collect();
                MachineData {
                    target,
                    buttons,
                    joltages,
                }
            })
            .collect::<Vec<_>>();
        if test {
            println!("Parsed machines:");
            for machine in &machines {
                println!("  target: {:0b} ", machine.target);
            }
        }
        machines
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut presses = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        'outer: for machine in input {
            queue.clear();
            visited.clear();

            // find min number of presses to turn on all required lights
            // Each queue element is a tuple: (current_state, remaining_clicks)
            queue.push_back((0, 0));

            while let Some((state, clicks)) = queue.pop_front() {
                for &b in &machine.buttons {
                    let new_state = state ^ b;
                    if new_state == machine.target {
                        presses += clicks + 1;
                        continue 'outer;
                    }
                    if visited.contains(&new_state) {
                        continue;
                    }
                    visited.insert(new_state);
                    queue.push_back((new_state, clicks + 1));
                }
            }
        }
        presses.to_string()
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
