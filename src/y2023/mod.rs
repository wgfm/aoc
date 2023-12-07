mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

use crate::problem::Solution;

pub fn solutions() -> anyhow::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(d1::Problem::new()?),
        Box::new(d2::Problem::new()?),
        Box::new(d3::Problem::new()?),
        Box::new(d4::Problem::new()?),
        Box::new(d5::Problem::new()?),
        Box::new(d6::Problem::new()?),
        Box::new(d7::Problem::new()?),
    ])
}
