extern crate ukodus;

use ukodus::{PROBLEMS, reduce};

fn main() {
    let iterations = iterations();
    for problem in PROBLEMS.iter_mut() {
        let problem = reduce(problem, iterations);
        println!("{}", problem);
    }
}

fn iterations() -> usize {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<usize>().
        unwrap_or(1000)
}
