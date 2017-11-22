extern crate ukodus;

use ukodus::{PROBLEMS, Reducer};

fn main() {
    let iterations = iterations();

    for problem in PROBLEMS.into_iter() {
        let reduced = Reducer::new(problem.clone()).reduce(iterations);
        println!("{}", reduced);
    }
}

fn iterations() -> usize {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<usize>().
        unwrap_or(1000)
}
