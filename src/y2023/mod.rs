mod d1;

use crate::problem::Solution;

pub fn solutions() -> anyhow::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![Box::new(d1::Problem::new()?)])
}
