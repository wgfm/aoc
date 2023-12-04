mod d1;
mod d2;
mod d3;

use crate::problem::Solution;

pub fn solutions() -> anyhow::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(d1::Problem::new()?),
        Box::new(d2::Problem::new()?),
        Box::new(d3::Problem::new()?),
    ])
}
