use std::collections::HashMap;

use anyhow::bail;

use crate::{problem::Solution, solution};

solution!(2015, 3);

impl Problem {
    fn visits(
        &self,
        mut input: impl Iterator<Item = char>,
    ) -> anyhow::Result<HashMap<(i64, i64), u64>> {
        let map = HashMap::from([((0, 0), 1)]);
        let (visited, _) = input.try_fold((map, (0, 0)), |(mut acc, (x, y)), e| {
            let new_pos = match e {
                '>' => (x + 1, y),
                '<' => (x - 1, y),
                '^' => (x, y + 1),
                'v' => (x, y - 1),
                _ => bail!("unknown input character: {}", e),
            };

            acc.entry(new_pos).and_modify(|c| *c += 1).or_insert(1);

            Ok((acc, new_pos))
        })?;

        Ok(visited)
    }
}

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let visited = self.visits(self.input.chars())?;

        Ok(format!("{}", visited.len()))
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let santa = self.input.chars().step_by(2);
        let robo_santa = self.input.chars().skip(1).step_by(2);

        let mut santa_visits = self.visits(santa)?;
        let robo_santa_visits = self.visits(robo_santa)?;

        santa_visits.extend(robo_santa_visits);

        Ok(format!("{}", santa_visits.len()))
    }
}
