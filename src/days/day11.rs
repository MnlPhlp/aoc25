use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use crate::types::DaySolver;

pub struct Solver;

#[derive(Debug, Clone)]
pub struct Machine<'a> {
    name: &'a str,
    connections: Vec<usize>,
}

#[derive(Clone)]
pub struct Input<'a> {
    machines: Vec<Machine<'a>>,
    indices: HashMap<&'a str, usize>,
}

fn count_paths(
    input: &Input,
    start: &str,
    end: &str,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let start_id = input.indices[start];
    let end_id = input.indices[end];
    let count = count_paths_inner(&input.machines, start_id, end_id, cache);
    println!("{start} -> {end}: {count}");
    count
}

fn count_paths_inner(
    machines: &[Machine<'_>],
    current: usize,
    end: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if current == end {
        return 1;
    }
    if let Some(count) = cache.get(&(current, end)) {
        return *count;
    }
    let mut sum = 0;
    for n in machines[current].connections.iter().copied() {
        sum += count_paths_inner(machines, n, end, cache);
    }
    cache.insert((current, end), sum);
    sum
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Input<'a>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let mut machines = Vec::new();
        let mut indices = HashMap::new();
        indices.insert("out", 0);
        machines.push(Machine {
            name: "out",
            connections: vec![],
        });
        for line in input.lines() {
            let (name, _) = line.split_once(": ").unwrap();
            let id = machines.len();
            indices.insert(name, id);
            machines.push(Machine {
                name,
                connections: vec![],
            });
        }
        for (i, line) in input.lines().enumerate() {
            let (_, connections) = line.split_once(": ").unwrap();
            for conn in connections.split(' ') {
                let conn_id = *indices.get(conn).unwrap();
                machines[i + 1].connections.push(conn_id);
            }
        }
        if test {
            println!("Machines: {machines:#?}");
        }
        Input { machines, indices }
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut cache = HashMap::new();
        count_paths(input, "you", "out", &mut cache).to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let mut paths = 0;
        let mut cache = HashMap::new();
        let fft_to_dac = count_paths(input, "fft", "dac", &mut cache);
        let dac_to_fft = count_paths(input, "dac", "fft", &mut cache);
        let svr_to_fft = count_paths(input, "svr", "fft", &mut cache);
        let svr_to_dac = count_paths(input, "svr", "dac", &mut cache);
        let fft_to_out = count_paths(input, "fft", "out", &mut cache);
        let dac_to_out = count_paths(input, "dac", "out", &mut cache);
        let svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;
        dbg!(svr_fft_dac_out);
        paths.to_string()
    }
}
