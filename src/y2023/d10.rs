use crate::{problem::Solution, solution};

solution!(2023, 10);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let maze: Vec<Vec<char>> = self
            .input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let polygon = polygon(&maze);
        Ok((polygon.len() / 2).to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let mut maze: Vec<Vec<char>> = self
            .input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let polygon = polygon(&maze);
        let polygon_set = polygon
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();

        for (y, row) in maze.iter_mut().enumerate() {
            for (x, c) in row.iter_mut().enumerate() {
                if !polygon_set.contains(&(x, y)) {
                    *c = ' ';
                }
            }
        }

        let mut num_internal = 0;

        for y in 0..maze.len() {
            let mut num_north_pipes = 0;
            for x in 0..maze[y].len() {
                let c = maze[y][x];
                match c {
                    '|' | 'L' | 'J' | 'S' => {
                        num_north_pipes += 1;
                    }
                    _ => {}
                }

                let counting = num_north_pipes % 2 == 1;
                if counting {
                    if c == ' ' {
                        num_internal += 1;
                        maze[y][x] = '!';
                    }
                }
            }
        }

        print_grid(&maze);

        Ok(num_internal.to_string())
    }
}

fn polygon(maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut start = (0, 0);
    // find 'S' in maze
    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
                dbg!(start);
            }
        }
    }

    let mut curr = first_step(&maze, start, (0, 0));
    let mut result = vec![curr];
    let mut prev = (0, 0);
    loop {
        let next = next_step(&maze, curr, prev);
        prev = curr;
        curr = next;

        result.push(curr);

        if curr == start {
            break;
        }
    }

    result
}

fn next_step(maze: &Vec<Vec<char>>, curr: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
    let opts = match maze[curr.1][curr.0] {
        '|' => vec![(curr.0, curr.1 + 1), (curr.0, curr.1 - 1)],
        '-' => vec![(curr.0 + 1, curr.1), (curr.0 - 1, curr.1)],
        'L' => vec![(curr.0 + 1, curr.1), (curr.0, curr.1 - 1)],
        'J' => vec![(curr.0 - 1, curr.1), (curr.0, curr.1 - 1)],
        '7' => vec![(curr.0 - 1, curr.1), (curr.0, curr.1 + 1)],
        'F' => vec![(curr.0 + 1, curr.1), (curr.0, curr.1 + 1)],
        _ => panic!("Unreachable from {:?} to {:?}", prev, curr),
    };

    opts.into_iter().find(|&c| c != prev).unwrap()
}

fn first_step(maze: &Vec<Vec<char>>, curr: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
    let candidate = (curr.0, curr.1 + 1);
    match maze.get(candidate.1).and_then(|r| r.get(candidate.0)) {
        // one down
        Some(&'|') | Some(&'L') | Some(&'J') => return candidate,
        _ => (),
    };

    let candidate = (curr.0, curr.1.saturating_sub(1));
    match maze.get(candidate.1).and_then(|r| r.get(candidate.0)) {
        // one up
        Some(&'|') | Some(&'7') | Some(&'F') => return candidate,
        _ => (),
    };

    let candidate = (curr.0 + 1, curr.1);
    match maze.get(candidate.1).and_then(|r| r.get(candidate.0)) {
        // one right
        Some(&'-') | Some(&'J') | Some(&'7') => return candidate,
        _ => (),
    };

    let candidate = (curr.0.saturating_sub(1), curr.1);
    match maze.get(candidate.1).and_then(|r| r.get(candidate.0)) {
        // one left
        Some(&'-') | Some(&'L') | Some(&'F') => return candidate,
        _ => (),
    };

    panic!("Unreachable from {:?} to {:?}", prev, curr);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
