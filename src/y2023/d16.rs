use std::collections::HashSet;

use crate::{problem::Solution, solution};

solution!(2023, 16);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let cave = parse(&self.input);
        let beam = Beam {
            pos: (0, 0),
            dir: (1, 0),
        };

        Ok(solve(&cave, beam).to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let cave = parse(&self.input);
        let mut answers = vec![];

        let bounding_box = (cave[0].len(), cave.len());
        let mut beams = vec![];

        for y in 0..bounding_box.1 {
            beams.push(Beam {
                pos: (0, y),
                dir: (1, 0),
            });

            beams.push(Beam {
                pos: (bounding_box.0 - 1, y),
                dir: (-1, 0),
            });
        }

        for x in 0..bounding_box.0 {
            beams.push(Beam {
                pos: (x, 0),
                dir: (0, 1),
            });

            beams.push(Beam {
                pos: (x, bounding_box.1 - 1),
                dir: (0, -1),
            });
        }

        for beam in beams {
            answers.push(solve(&cave, beam));
        }

        Ok(answers.iter().max().unwrap().to_string())
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Beam {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Beam {
    fn step(&mut self, bounding_box: (usize, usize)) -> bool {
        self.pos.0 = self.pos.0.wrapping_add_signed(self.dir.0);
        self.pos.1 = self.pos.1.wrapping_add_signed(self.dir.1);

        (0..bounding_box.0).contains(&self.pos.0) && (0..bounding_box.1).contains(&self.pos.1)
    }
}

fn solve(cave: &[Vec<char>], beam: Beam) -> usize {
    let bounding_box = (cave[0].len(), cave.len());

    let mut energized = vec![vec![false; cave[0].len()]; cave.len()];

    let mut beams = vec![beam];

    let mut seen = HashSet::new();

    for _ in 0..1000 {
        let mut next_beams = vec![];
        for mut beam in beams {
            if seen.contains(&beam) {
                continue;
            }
            seen.insert(beam.clone());
            energized[beam.pos.1][beam.pos.0] = true;

            match cave[beam.pos.1][beam.pos.0] {
                '.' => {}
                '|' => match beam.dir {
                    (x, 0) => {
                        let mut split_beam = beam.clone();
                        split_beam.dir = (0, 1);
                        beam.dir = (0, -1);

                        if split_beam.step(bounding_box) {
                            next_beams.push(split_beam);
                        }
                    }
                    (0, y) => {}
                    _ => unreachable!(),
                },
                '-' => match beam.dir {
                    (x, 0) => {}
                    (0, y) => {
                        let mut split_beam = beam.clone();
                        split_beam.dir = (1, 0);
                        beam.dir = (-1, 0);

                        if split_beam.step(bounding_box) {
                            next_beams.push(split_beam);
                        }
                    }
                    _ => unreachable!(),
                },
                '/' => {
                    match beam.dir {
                        (1, 0) => {
                            beam.dir = (0, -1);
                        }
                        (0, 1) => {
                            beam.dir = (-1, 0);
                        }
                        (-1, 0) => {
                            beam.dir = (0, 1);
                        }
                        (0, -1) => {
                            beam.dir = (1, 0);
                        }
                        _ => unreachable!(),
                    };
                }
                '\\' => {
                    match beam.dir {
                        (1, 0) => {
                            beam.dir = (0, 1);
                        }
                        (0, 1) => {
                            beam.dir = (1, 0);
                        }
                        (-1, 0) => {
                            beam.dir = (0, -1);
                        }
                        (0, -1) => {
                            beam.dir = (-1, 0);
                        }
                        _ => unreachable!(),
                    };
                }
                _ => unreachable!(),
            }

            if beam.step(bounding_box) {
                next_beams.push(beam);
            }
        }
        beams = next_beams;
    }

    let answer = energized.iter().flatten().filter(|&&b| b).count();
    answer
}
