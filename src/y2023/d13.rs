use crate::{problem::Solution, solution};

solution!(2023, 13);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let areas = parse(&self.input);
        let mut answer = 0;

        for area in areas {
            if let Some(mirror_row) = find_mirror(&area, 0) {
                answer += mirror_row;
            }

            if let Some(mirror_row) = find_mirror(&transpose(area), 0) {
                answer += mirror_row * 100;
            }
        }

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let areas = parse(&self.input);
        let mut answer = 0;

        for area in areas {
            if let Some(mirror_row) = find_mirror(&area, 1) {
                answer += mirror_row;
            }

            if let Some(mirror_row) = find_mirror(&transpose(area), 1) {
                answer += mirror_row * 100;
            }
        }

        Ok(answer.to_string())
    }
}

fn find_mirror(area: &[Vec<char>], smudges: u64) -> Option<usize> {
    let mut row_candidates = vec![0; area[0].len()];
    row_candidates[0] = 8;

    for row in area.iter() {
        for i in 1..row.len() {
            row[0..i]
                .iter()
                .rev()
                .zip(row[i..].iter())
                .for_each(|(left, right)| {
                    if *left != *right {
                        row_candidates[i] += 1;
                    }
                });
        }
    }

    row_candidates
        .iter()
        .enumerate()
        .find(|(_, &b)| b == smudges)
        .map(|(i, _)| i)
}

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mirror_row() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let area = input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<_>>();

        assert_eq!(find_mirror(&area, 0), Some(5));
    }
}
