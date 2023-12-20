use crate::{problem::Solution, solution};

solution!(2023, 18);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let steps = self.input.lines().map(Step::parse).collect::<Vec<_>>();

        let answer = solve(&steps);

        Ok(answer.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let steps = self.input.lines().map(Step::parse2).collect::<Vec<_>>();
        let answer = solve(&steps);
        Ok(answer.to_string())
    }
}

fn solve(steps: &[Step]) -> i64 {
    let mut positions = vec![(0, 0)];

    let mut num_steps = 0;
    for step in steps {
        let (x, y) = positions.last().unwrap().to_owned();
        let (dx, dy) = step.dir;
        num_steps += step.len;
        positions.push((x + dx * step.len, y + dy * step.len));
    }

    dbg!(&positions);

    let mut area = 0;
    for win in positions.windows(2) {
        let (x1, y1) = win[0];
        let (x2, y2) = win[1];

        area += x1 * y2 - x2 * y1;
    }

    let answer = area / 2 - num_steps / 2 + 1 + num_steps;

    answer
}

struct Step {
    dir: (i64, i64),
    len: i64,
}

impl Step {
    fn parse(input: &str) -> Step {
        let (dir, input) = input.split_once(" ").unwrap();

        let dir = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("invalid direction"),
        };

        let (len, color) = input.split_once(" ").unwrap();
        let len = len.parse().unwrap();

        Step { dir, len }
    }

    fn parse2(input: &str) -> Step {
        let (_, code) = input.split_once("#").unwrap();
        let code = code.trim_end_matches(")");
        let (len, dir) = code.split_at(5);
        assert_eq!(dir.len(), 1);

        let dir = match dir {
            "0" => (1, 0),
            "1" => (0, 1),
            "2" => (-1, 0),
            "3" => (0, -1),
            _ => panic!("invalid direction"),
        };

        let len = i64::from_str_radix(len, 16).unwrap();

        Step { dir, len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_a() {
        let problem = Problem::with_input(INPUT);

        assert_eq!(problem.part_a().unwrap(), "62".to_string());
    }
}
