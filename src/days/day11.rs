use std::collections::{HashMap, VecDeque};

use crate::types::DaySolver;

pub struct Solver;

#[derive(Debug)]
pub struct Machine<'a> {
    id: usize,
    name: &'a str,
    connections: Vec<usize>,
}

pub struct Input<'a> {
    machines: Vec<Machine<'a>>,
    indices: HashMap<&'a str, usize>,
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Input<'a>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let mut machines = Vec::new();
        let mut indices = HashMap::new();
        indices.insert("out", 0);
        machines.push(Machine {
            id: 0,
            name: "out",
            connections: vec![],
        });
        for line in input.lines() {
            let (name, _) = line.split_once(": ").unwrap();
            let id = machines.len();
            indices.insert(name, id);
            machines.push(Machine {
                id,
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
        let mut paths = 0;
        let start = input.indices["you"];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        // let mut visited = vec![false; input.len()];
        while let Some(current) = queue.pop_front() {
            if input.machines[current].name == "out" {
                paths += 1;
                continue;
            }
            // if visited[current] {
            //     continue;
            // }
            // visited[current] = true;
            for &conn in &input.machines[current].connections {
                // if !visited[conn] {
                queue.push_back(conn);
                // }
            }
        }
        paths.to_string()
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
