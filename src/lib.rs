extern crate itertools;
extern crate rand;
extern crate rayon;

mod puzzle;
mod problems;
mod solver;
mod reducer;

pub use problems::PROBLEMS;
pub use puzzle::Puzzle;
pub use solver::Solver;
pub use reducer::Reducer;
