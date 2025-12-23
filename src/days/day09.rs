use crate::types::DaySolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u64,
    y: u64,
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
        let mut max_size = 0u64;
        let mut a = &Point { x: 0, y: 0 };
        let mut b = &Point { x: 0, y: 0 };
        for (i, one) in input.iter().enumerate() {
            for two in &input[(i + 1)..] {
                let size = (one.x.abs_diff(two.x) + 1) * (one.y.abs_diff(two.y) + 1);
                if size > max_size && is_valid_area(one, two, input) {
                    a = one;
                    b = two;
                    max_size = size;
                }
            }
        }
        if test {
            print_rectangle(input, a, b);
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
// i.e. no other points from input lie inside the rectangle defined by one and two
fn is_valid_area(one: &Point, two: &Point, input: &[Point]) -> bool {
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

    for i in 0..input.len() {
        let point = &input[i];
        // skip the corners of the rectangle
        if point == one || point == two {
            continue;
        }
        let last_point = if i == 0 {
            &input[input.len() - 1]
        } else {
            &input[i - 1]
        };

        // check if the line between this point and the last one is crossing the rectangle
        let line_min_x = point.x.min(last_point.x);
        let line_max_x = point.x.max(last_point.x);
        let line_min_y = point.y.min(last_point.y);
        let line_max_y = point.y.max(last_point.y);

        // check if completely outside, touching one edge is allowed
        if line_max_x <= min_x || line_min_x >= max_x || line_max_y <= min_y || line_min_y >= max_y
        {
            continue;
        }
        // check if along one edge
        if line_min_x == line_max_x && (line_min_x == max_x || line_min_x == min_x) {
            continue;
        }
        if line_min_y == line_max_y && (line_min_y == max_y || line_min_y == min_y) {
            continue;
        }
        return false;
    }
    true
}
