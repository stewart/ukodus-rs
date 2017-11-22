extern crate ukodus;

use ukodus::{PROBLEMS};

fn main() {
    for problem in PROBLEMS.iter_mut() {
        println!("{}", problem);
    }
}

fn iterations() -> usize {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<usize>().
        unwrap_or(1000)
}
