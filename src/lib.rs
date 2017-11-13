extern crate itertools;

mod puzzle;
mod problems;
mod solver;

pub use problems::PROBLEMS;
pub use puzzle::Puzzle;
pub use solver::solve;
