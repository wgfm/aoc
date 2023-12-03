mod d1;
mod d2;

use crate::problem::Solution;

pub fn solutions() -> anyhow::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(d1::Problem::new()?),
        Box::new(d2::Problem::new()?),
    ])
}
