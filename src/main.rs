extern crate ukodus;

use ukodus::{PROBLEMS};

fn main() {
    for problem in PROBLEMS.iter() {
        println!("{}", problem);
    }
}
