use anyhow::Result;

use crate::y2015;
use crate::y2020;
use crate::y2023;

#[macro_export]
macro_rules! solution {
    ($year:expr,$day:expr) => {
        pub struct Problem {
            input: String,
        }

        impl Problem {
            pub fn new() -> Result<Self, anyhow::Error> {
                Ok(Self {
                    input: crate::site::get_puzzle_input($year, $day)?,
                })
            }

            #[allow(dead_code)]
            pub fn with_input(input: &str) -> Self {
                Self {
                    input: input.to_string(),
                }
            }
        }
    };
}

pub trait Solution {
    fn part_a(&self) -> Result<String>;
    fn part_b(&self) -> Result<String>;
}

pub fn problems_for_year(year: u64) -> Result<Vec<Box<dyn Solution>>> {
    match year {
        2015 => y2015::solutions(),
        2020 => y2020::solutions(),
        2023 => y2023::solutions(),
        _ => Ok(vec![]),
    }
}

pub struct NoSolution {}

impl NoSolution {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for NoSolution {
    fn part_a(&self) -> Result<String> {
        unimplemented!("No solution yet!");
    }

    fn part_b(&self) -> Result<String> {
        unimplemented!("No solution yet!");
    }
}
