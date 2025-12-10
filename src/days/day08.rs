use crate::types::DaySolver;

pub struct Point {
    x: u64,
    y: u64,
    z: u64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Point>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let coords: Vec<_> = line.split(',').map(|num| num.parse().unwrap()).collect();
                Point {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                }
            })
            .collect()
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let iterations = if test { 10 } else { 1000 };
        // each point is indexed by its position in the input vector
        // group 0 means no group assigned yet
        let mut group_size: Vec<usize> = vec![0];
        let mut group_id = vec![0; input.len()];

        // distances between points and indices of the points, sorted by distance
        let mut min_distances: Vec<(usize, usize, u64)> = Vec::with_capacity(iterations + 1);
        for (i, one) in input.iter().enumerate() {
            for (j, two) in input.iter().enumerate().skip(i + 1) {
                let distance = (one.x.abs_diff(two.x).pow(2)
                    + one.y.abs_diff(two.y).pow(2)
                    + one.z.abs_diff(two.z).pow(2))
                .isqrt();
                match min_distances.binary_search_by(|&(_, _, d)| d.cmp(&distance)) {
                    Ok(pos) | Err(pos) => {
                        min_distances.insert(pos, (i, j, distance));
                        if min_distances.len() > iterations {
                            min_distances.pop();
                        }
                    }
                }
            }
        }
        if test {
            for (i, j, d) in &min_distances {
                println!("{} -> {}: {d}", input[*i], input[*j]);
            }
        }
        // build groups based on min distances
        for (i, j, _) in min_distances {
            let group_one = group_id[i];
            let group_two = group_id[j];
            if group_one == 0 && group_two == 0 {
                // create new group
                let new_group = group_size.len();
                group_size.push(2);
                group_id[i] = new_group;
                group_id[j] = new_group;
            } else if group_one != 0 && group_two == 0 {
                // add j to group of i
                group_size[group_one] += 1;
                group_id[j] = group_one;
            } else if group_one == 0 && group_two != 0 {
                // add i to group of j
                group_size[group_two] += 1;
                group_id[i] = group_two;
            } else if group_one != group_two {
                // merge groups
                group_size[group_one] += group_size[group_two];
                for gid in &mut group_id {
                    if *gid == group_two {
                        *gid = group_one;
                    }
                }
            }
        }
        if test {
            for (g, members) in group_size.iter().enumerate().skip(1) {
                println!("Group {g}: {members}");
            }
        }
        // find 3 largest groups
        let mut largest_sizes = [0; 3];
        for &size in &group_size {
            if size > largest_sizes[0] {
                largest_sizes[2] = largest_sizes[1];
                largest_sizes[1] = largest_sizes[0];
                largest_sizes[0] = size;
            } else if size > largest_sizes[1] {
                largest_sizes[2] = largest_sizes[1];
                largest_sizes[1] = size;
            } else if size > largest_sizes[2] {
                largest_sizes[2] = size;
            }
        }
        if test {
            println!(
                "Largest sizes: {}, {}, {}",
                largest_sizes[0], largest_sizes[1], largest_sizes[2]
            );
        }
        let result: usize = largest_sizes.iter().product();
        result.to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        // find the shortest connection for each point to any other point
        let mut shortes_connection: Vec<(usize, usize, u64)> = Vec::with_capacity(input.len());
        for (i, one) in input.iter().enumerate() {
            let mut min_distance = u64::MAX;
            let mut min_pair = 0;
            for (j, two) in input.iter().enumerate() {
                // skip self and already connected points
                if i == j || shortes_connection.get(j).is_some_and(|pair| pair.0 == i) {
                    continue;
                }
                let distance = (one.x.abs_diff(two.x).pow(2)
                    + one.y.abs_diff(two.y).pow(2)
                    + one.z.abs_diff(two.z).pow(2))
                .isqrt();
                if distance < min_distance {
                    min_distance = distance;
                    min_pair = j;
                }
            }
            shortes_connection.push((i, min_pair, min_distance));
        }
        shortes_connection.sort_unstable_by(|a, b| a.2.cmp(&b.2));
        let last_connection = shortes_connection.last().unwrap();
        println!(
            "Last connection: {} -> {}: {}",
            input[last_connection.0], input[last_connection.1], last_connection.2
        );
        let result = input[last_connection.0].x * input[last_connection.1].x;
        result.to_string()
    }
}
