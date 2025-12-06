use crate::types::DaySolver;

pub struct Solver;

fn remove_rolls(input: &mut [Vec<bool>], remove: bool) -> u32 {
    let mut count = 0;
    let height = input.len();
    let width = input[0].len();
    for y in 0..height {
        for x in 0..width {
            if !input[y][x] {
                continue;
            }
            let coords = [
                (x as isize, y as isize - 1),     // Up
                (x as isize + 1, y as isize - 1), // UP Right
                (x as isize + 1, y as isize),     // Right
                (x as isize + 1, y as isize + 1), // Down Right
                (x as isize, y as isize + 1),     // Down
                (x as isize - 1, y as isize + 1), // Down Left
                (x as isize - 1, y as isize),     // Left
                (x as isize - 1, y as isize - 1), // Up Left
            ];
            let neighbour_count = coords
                .iter()
                .filter(|(nx, ny)| {
                    if *nx >= 0 && *nx < width as isize && *ny >= 0 && *ny < height as isize {
                        input[*ny as usize][*nx as usize]
                    } else {
                        false
                    }
                })
                .count();
            if neighbour_count < 4 {
                count += 1;
                if remove {
                    input[y][x] = false;
                }
            }
        }
    }
    count
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<bool>>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
            .lines()
            .map(|l| l.as_bytes().iter().map(|b| *b == b'@').collect())
            .collect()
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut input = input.clone();
        let count = remove_rolls(&mut input, false);
        count.to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let mut count = 0;
        let mut input = input.clone();
        loop {
            let removed = remove_rolls(&mut input, true);
            if removed == 0 {
                break;
            }
            count += removed;
        }
        count.to_string()
    }
}
