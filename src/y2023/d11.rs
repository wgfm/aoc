use num::abs;

use crate::{problem::Solution, solution};

solution!(2023, 11);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let answer = solve(&self.input, 1);
        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let answer = solve(&self.input, 1000000);
        Ok(answer.to_string())
    }
}

fn solve(input: &str, factor: usize) -> isize {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows_without_galaxies = universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let cols_without_galaxies = (0..universe[0].len())
        .filter(|&x| universe.iter().all(|row| row[x] == '.'))
        .collect::<Vec<_>>();

    let mut galaxies = vec![];
    let mut y_offset = 0;
    for (y, row) in universe.iter().enumerate() {
        let mut x_offset = 0;
        if let Some(_) = rows_without_galaxies.iter().find(|&&i| i == y) {
            y_offset += 1 * factor;
            continue;
        }

        for (x, c) in row.iter().enumerate() {
            if let Some(_) = cols_without_galaxies.iter().find(|&&i| i == x) {
                x_offset += 1 * factor;
                continue;
            }

            if *c == '#' {
                galaxies.push((x + x_offset, y + y_offset));
            }
        }
    }

    let mut answer = 0;
    for (i, (x0, y0)) in galaxies.iter().enumerate() {
        for (x1, y1) in galaxies[i + 1..].iter() {
            answer += abs(*x0 as isize - *x1 as isize) + abs(*y0 as isize - *y1 as isize);
        }
    }

    answer
}
