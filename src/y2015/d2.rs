use std::cmp::min;

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{preceded, tuple},
    Finish, IResult,
};

use crate::{problem::Solution, solution};

solution!(2015, 2);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let boxes: Vec<RightRectangularPrism> = self
            .input
            .lines()
            .map(|l| {
                all_consuming(RightRectangularPrism::parse)(l)
                    .finish()
                    .unwrap()
                    .1
            })
            .collect();

        let answer: u64 = boxes
            .into_iter()
            .map(|RightRectangularPrism { x, y, z }| {
                let (s1, s2, s3) = ((x * y), (y * z), (z * x));
                let m = min(s1, min(s2, s3));
                m + 2 * s1 + 2 * s2 + 2 * s3
            })
            .sum();

        Ok(format!("{}", answer))
    }

    fn part_b(&self) -> anyhow::Result<String> {
        todo!()
    }
}

struct RightRectangularPrism {
    x: u64,
    y: u64,
    z: u64,
}

impl RightRectangularPrism {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                nom::character::complete::u64,
                preceded(tag("x"), nom::character::complete::u64),
                preceded(tag("x"), nom::character::complete::u64),
            )),
            |(x, y, z)| Self { x, y, z },
        )(input)
    }
}
