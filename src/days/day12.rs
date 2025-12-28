use crate::types::DaySolver;

pub struct Solver;

#[derive(Debug)]
pub struct Input {
    shapes: [[[bool; 3]; 3]; 6],
    regions: Vec<Region>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<(usize, u32)>,
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Input;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let mut shapes = [[[false; 3]; 3]; 6];
        let mut regions = Vec::new();
        let mut lines = input.lines();
        // first parse the 5 shapes
        for i in 0..=5 {
            let _ = lines.next(); // skip id line
            for y in 0..3 {
                let line = lines.next().unwrap();
                for (x, c) in line.chars().enumerate() {
                    shapes[i][y][x] = c == '#';
                }
            }
            let _ = lines.next(); // skip empty line
        }
        // then parse the regions
        for line in lines {
            let (dims, shapes) = line.split_once(": ").unwrap();
            let (width, height) = dims.split_once('x').unwrap();
            let width: usize = width.parse().unwrap();
            let height: usize = height.parse().unwrap();
            let shapes = shapes
                .split(" ")
                .enumerate()
                .map(|(i, s)| (i, s.parse().unwrap()))
                .filter(|&(_, count)| count > 0)
                .collect();
            regions.push(Region {
                width,
                height,
                shapes,
            });
        }
        if test {
            dbg!(&shapes);
            dbg!(&regions);
        }
        Input { shapes, regions }
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let mut fitting = 0;
        for region in &input.regions {
            // check if the regions fit without any overlaps -> trivially fits
            let total_shapes = region.shapes.iter().map(|&(_, count)| count).sum::<u32>();
            let fit_width = region.width / 3;
            let fit_height = region.height / 3;
            if total_shapes as usize <= fit_width * fit_height {
                fitting += 1;
                continue;
            }
            // check if the total filled area is to big -> cannot fit
            let total_area: usize = region
                .shapes
                .iter()
                .map(|&(shape_idx, count)| {
                    let shape = &input.shapes[shape_idx];
                    let shape_area = shape.iter().flatten().filter(|&&b| b).count();
                    shape_area * (count as usize)
                })
                .sum();
            if total_area > region.width * region.height {
                continue;
            }
            todo!("Implement actual fitting algorithm");
        }

        fitting.to_string()
    }

    fn solve2(&self, _input: &Self::Input, _test: bool) -> String {
        "Not implemented".to_string()
    }
}
