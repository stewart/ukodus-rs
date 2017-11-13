extern crate itertools;
extern crate rand;

mod puzzle;
mod problems;
mod solver;
mod reducer;

pub use problems::PROBLEMS;
pub use puzzle::Puzzle;
pub use solver::solve;
pub use reducer::reduce;
