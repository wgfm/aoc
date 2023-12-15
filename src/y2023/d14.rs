use std::cmp::{max, min};

use crate::{problem::Solution, solution};

solution!(2023, 14);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut platform = parse(&self.input);
        platform.tilt_north();

        Ok(platform.north_load().to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let mut platform = parse(&self.input);

        let mut seen = std::collections::HashMap::new();

        let mut loop_point = 0;
        let mut start = 0;
        for i in 0..1_000_000_000 {
            seen.insert(platform.contents.clone(), i);

            platform.tilt_north();
            platform.tilt_west();
            platform.tilt_south();
            platform.tilt_east();

            if let Some(from) = seen.get(&platform.contents) {
                loop_point = i;
                start = *from;
                break;
            }
        }

        let mut inverse_seen = vec![vec![]; seen.len()];
        for (k, v) in seen {
            inverse_seen[v] = k;
        }

        let w = start + (1_000_000_000 - start) % (loop_point - start + 1);
        let map = inverse_seen.get(w).unwrap();
        let p2 = Platform {
            contents: map.clone(),
            ..platform
        };

        Ok(p2.north_load().to_string())
    }
}

fn parse(input: &str) -> Platform {
    let contents: Vec<Vec<Contents>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Contents::EmptySpace,
                    '#' => Contents::CubeShapedRock,
                    'O' => Contents::RoundedRock,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let width = contents[0].len();
    let height = contents.len();

    Platform {
        contents,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Contents {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
}

struct Platform {
    contents: Vec<Vec<Contents>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn tilt(&mut self, y_dir: isize, x_dir: isize) {
        let mut changed = true;
        while changed {
            changed = false;
            let y_range =
                max(0, 0 - y_dir)..min(self.height as isize, self.height as isize - y_dir);
            for y in y_range {
                let y = y as usize;
                let x_range =
                    max(0, 0 - x_dir)..min(self.width as isize, self.width as isize - x_dir);
                for x in x_range {
                    let x = x as usize;
                    let contents = self.contents[y][x];
                    if contents == Contents::RoundedRock {
                        if self.contents[(y as isize + y_dir) as usize]
                            [(x as isize + x_dir) as usize]
                            == Contents::EmptySpace
                        {
                            self.contents[(y as isize + y_dir) as usize]
                                [(x as isize + x_dir) as usize] = Contents::RoundedRock;
                            self.contents[y][x] = Contents::EmptySpace;
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        self.tilt(-1, 0);
    }

    fn tilt_south(&mut self) {
        self.tilt(1, 0);
    }

    fn tilt_east(&mut self) {
        self.tilt(0, 1);
    }

    fn tilt_west(&mut self) {
        self.tilt(0, -1);
    }

    fn north_load(&self) -> usize {
        let mut answer = 0;
        for (y, row) in self.contents.iter().enumerate() {
            let row_weight = self.contents.len() - y;
            for contents in row {
                if *contents == Contents::RoundedRock {
                    answer += row_weight;
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loopy() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let problem = Problem::with_input(input);

        assert_eq!(problem.part_b().unwrap(), "64".to_string());
    }
}
