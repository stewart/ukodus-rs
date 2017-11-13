extern crate ukodus;

use ukodus::{PROBLEMS, reduce};

fn main() {
    for problem in PROBLEMS.iter_mut() {
        reduce(problem);
        println!("{}", problem);
    }

    println!("{}", iterations());
}

fn iterations() -> u32 {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<u32>().
        unwrap_or(1000)
}
