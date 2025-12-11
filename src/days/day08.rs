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
                let distance = one.x.abs_diff(two.x).pow(2)
                    + one.y.abs_diff(two.y).pow(2)
                    + one.z.abs_diff(two.z).pow(2);

                // skip search if distance is already larger than the largest in min_distances
                if distance > min_distances.last().map_or(u64::MAX, |&(_, _, d)| d) {
                    if min_distances.len() < iterations {
                        min_distances.push((i, j, distance));
                    }
                    continue;
                }
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
        // the task is to find the last connection made when always connecting the closest pairs
        // until all points are connected
        // this means we need to find the longest distance among the shortest distances for each point

        let mut max_distance = 0;
        let mut max_a = 0;
        let mut max_b = 0;

        // find the longest one of the shortest connection for each point
        'outer: for (i, one) in input.iter().enumerate() {
            let mut min_distance = u64::MAX;
            let mut min_pair = 0;
            // find the closest point to 'one'
            for (j, two) in input.iter().enumerate() {
                if i == j {
                    continue;
                }
                let distance = one.x.abs_diff(two.x).pow(2)
                    + one.y.abs_diff(two.y).pow(2)
                    + one.z.abs_diff(two.z).pow(2);
                if distance < max_distance {
                    // no need to check further, this will never be the longest shortest distance
                    continue 'outer;
                }
                if distance < min_distance {
                    min_distance = distance;
                    min_pair = j;
                }
            }
            if min_distance > max_distance {
                max_distance = min_distance;
                max_a = i;
                max_b = min_pair;
            }
        }

        println!(
            "Last connection: {} -> {}: {}",
            input[max_a],
            input[max_b],
            max_distance.isqrt()
        );
        let result = input[max_a].x * input[max_b].x;
        result.to_string()
    }
}
