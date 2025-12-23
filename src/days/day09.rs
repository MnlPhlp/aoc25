use crate::types::DaySolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
}

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Point>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect()
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut max_size = 0u64;
        for (i, one) in input.iter().enumerate() {
            for two in &input[(i + 1)..] {
                let size = (one.x.abs_diff(two.x) + 1) * (one.y.abs_diff(two.y) + 1);
                if size > max_size {
                    max_size = size;
                }
            }
        }
        max_size.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        // Pre-compute segments sorted by min_x for efficient checks
        let mut segments: Vec<Segment> = (0..input.len())
            .map(|i| {
                let p1 = i;
                let p2 = if i == 0 { input.len() - 1 } else { i - 1 };
                let point = &input[p1];
                let last_point = &input[p2];
                Segment {
                    min_x: point.x.min(last_point.x),
                    max_x: point.x.max(last_point.x),
                    min_y: point.y.min(last_point.y),
                    max_y: point.y.max(last_point.y),
                }
            })
            .collect();
        segments.sort_unstable_by_key(|s| s.min_x);

        let mut max_size = 0u64;
        let mut best_pair = (0, 0);

        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                let one = &input[i];
                let two = &input[j];
                let size = (one.x.abs_diff(two.x) + 1) * (one.y.abs_diff(two.y) + 1);
                if size > max_size && is_valid_area_fast(one, two, &segments) {
                    best_pair = (i, j);
                    max_size = size;
                }
            }
        }

        if test && max_size > 0 {
            print_rectangle(input, &input[best_pair.0], &input[best_pair.1]);
        }
        max_size.to_string()
    }
}

fn print_rectangle(input: &[Point], one: &Point, two: &Point) {
    let max_x = input.iter().map(|p| p.x).max().unwrap() + 2;
    let max_y = input.iter().map(|p| p.y).max().unwrap() + 2;
    let rect_min = Point {
        x: one.x.min(two.x),
        y: one.y.min(two.y),
    };
    let rect_max = Point {
        x: one.x.max(two.x),
        y: one.y.max(two.y),
    };
    for y in 0..max_y {
        for x in 0..max_x {
            let point = Point { x, y };
            if input.iter().any(|p| p.x == point.x && p.y == point.y) {
                print!("O");
            } else if (x >= rect_min.x && x <= rect_max.x) && (y >= rect_min.y && y <= rect_max.y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// check if the rectangle lies completely inside the area defined by the points in input
// Uses pre-sorted segments for faster filtering
fn is_valid_area_fast(one: &Point, two: &Point, segments: &[Segment]) -> bool {
    let (min_x, max_x) = if one.x < two.x {
        (one.x, two.x)
    } else {
        (two.x, one.x)
    };
    let (min_y, max_y) = if one.y < two.y {
        (one.y, two.y)
    } else {
        (two.y, one.y)
    };

    // find first segment that could intersect (max_x > min_x of rect)
    // segments are sorted by min_x
    let start = segments
        .iter()
        .position(|seg| seg.max_x >= min_x)
        .unwrap_or(segments.len());

    for seg in &segments[start..] {
        // Early exit: if segment's min_x >= max_x of rect, no more can intersect
        if seg.min_x >= max_x {
            break;
        }

        // check if completely outside, touching one edge is allowed
        if seg.max_x <= min_x || seg.min_x >= max_x || seg.max_y <= min_y || seg.min_y >= max_y {
            continue;
        }
        // check if along one edge
        if seg.min_x == seg.max_x && (seg.min_x == max_x || seg.min_x == min_x) {
            continue;
        }
        if seg.min_y == seg.max_y && (seg.min_y == max_y || seg.min_y == min_y) {
            continue;
        }

        return false;
    }
    true
}
