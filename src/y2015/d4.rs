use anyhow::bail;

use crate::{problem::Solution, solution};

solution!(2015, 4);

impl Problem {
    fn solve(&self, mask: u32) -> anyhow::Result<u64> {
        for i in 0..500000000 {
            let to_mine = format!("{}{}", self.input.trim(), i);
            let hash = md5::compute(&to_mine);
            let first_four_bytes = u32::from_be_bytes(hash[0..4].try_into()?);
            if first_four_bytes & mask == 0 {
                return Ok(i);
            }
        }

        bail!("hash not found")
    }
}

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        self.solve(0xFFFFF000).map(|i| i.to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        self.solve(0xFFFFFF00).map(|i| i.to_string())
    }
}
