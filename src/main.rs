extern crate itertools;

mod puzzle;
mod problems;
mod solver;

use problems::PROBLEMS;

fn main() {
    for problem in PROBLEMS.iter() {
        println!("{}", problem);
    }
}
