use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
};

use crate::{problem::Solution, solution};

solution!(2023, 21);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let (map, start) = parse(&self.input);
        dbg!(map[0].len(), map.len(), start);
        Ok(solve_a(&map, start, 64).to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let (map, start) = parse(&self.input);
        let even_tile = solve_tile(&map, (0, 0));
        let odd_tile = solve_tile(&map, (1, 0));

        let left = solve_a(&map, (130, 65), 131);
        let right = solve_a(&map, (0, 65), 131);
        let bottom = solve_a(&map, (65, 130), 131);
        let top = solve_a(&map, (65, 0), 131);

        let top_right = solve_a(&map, (130, 0), 131);
        let top_left = solve_a(&map, (0, 0), 131);
        let bottom_right = solve_a(&map, (130, 130), 131);
        let bottom_left = solve_a(&map, (0, 130), 131);

        let total_steps = 26501365;
        let grid_size = 131;
        let num_grids = total_steps / grid_size;

        let bulk = (num_grids * num_grids) * (even_tile + odd_tile);

        dbg!(
            left,
            right,
            top,
            bottom,
            top_right,
            top_left,
            bottom_right,
            bottom_left,
            bulk
        );

        // Did the calctulation by hand

        unimplemented!()
    }
}

// Map is 65x65. There will be a tile pattern that alternates for every map repeat, both vertically
// and horizontally. We need to solve for the even and the odd repeat, as well as for the edges.
// Unfortunately, there are some tiles that are not reachable, so we can't overlay the map with
// alternating O and . to short-circuit the pattern.

fn solve_tile(map: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut visited = HashMap::new();
    let (max_x, max_y) = (map[0].len(), map.len());

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1));

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains_key(&(x, y)) {
            continue;
        }

        visited.insert((x, y), map[y][x]);

        if map[y][x] == '#' {
            continue;
        }

        queue.push_back((min(x + 1, max_x - 1), y));
        queue.push_back((x.saturating_sub(1), y));
        queue.push_back((x, min(y + 1, max_y - 1)));
        queue.push_back((x, y.saturating_sub(1)));
    }

    visited
        .iter()
        .filter(|((x, y), _)| (x + y) % 2 == (start.0 + start.1) % 2)
        .filter(|(_, c)| **c == '.')
        .count()
}

fn solve_a(map: &Vec<Vec<char>>, start: (usize, usize), num_steps: usize) -> usize {
    let mut visited = HashMap::new();
    let (max_x, max_y) = (map[0].len(), map.len());

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if steps >= num_steps + 1 {
            continue;
        }

        if visited.contains_key(&(x, y)) {
            continue;
        }

        visited.insert((x, y), map[y][x]);

        if map[y][x] == '#' {
            continue;
        }

        queue.push_back((min(x + 1, max_x - 1), y, steps + 1));
        queue.push_back((x.saturating_sub(1), y, steps + 1));
        queue.push_back((x, min(y + 1, max_y - 1), steps + 1));
        queue.push_back((x, y.saturating_sub(1), steps + 1));
    }

    // plot_map(map, &visited, start);
    visited
        .iter()
        .filter(|((x, y), _)| (x + y) % 2 == (start.0 + start.1) % 2)
        .filter(|(_, c)| **c == '.')
        .count()
}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == 'S' {
                start = (x, y);
            }
        })
    });

    map[start.1][start.0] = '.';

    (map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d21_parse() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let (map, start) = parse(input);
        assert_eq!(solve_a(&map, start, 6), 16);
    }
}

fn plot_map(map: &Vec<Vec<char>>, visited: &HashMap<(usize, usize), char>, start: (usize, usize)) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Some('.') = visited.get(&(x, y)) {
                if (x + y) % 2 == (start.0 + start.1) % 2 {
                    print!("O");
                } else {
                    print!(".");
                }
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}
