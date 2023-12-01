use crate::problem::{NoSolution, Solution};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn b<T: Solution>(t: T) -> Box<T> {
    Box::new(t)
}

pub fn solutions() -> anyhow::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        b(NoSolution::new()),
        b(d2::Problem::new()?),
        b(d3::Problem::new()?),
        b(d4::Problem::new()?),
        b(d5::Problem::new()?),
    ])
}
