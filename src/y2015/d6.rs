use crate::{problem::Solution, solution};

solution!(2015, 6);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let lights = vec![vec![false; 1000]; 1000];

        let mut count = 0;

        for col in &lights {
            for _ in col {
                count += 1;
            }
        }

        println!("{}", count);

        Ok("".into())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        todo!()
    }
}
