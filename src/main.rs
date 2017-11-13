extern crate itertools;

mod puzzle;
mod problems;

use problems::PROBLEMS;

fn main() {
    for problem in PROBLEMS.iter() {
        println!("{}", problem);
    }
}
