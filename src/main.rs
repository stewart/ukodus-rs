extern crate ukodus;
extern crate rayon;

use ukodus::{PROBLEMS, Reducer};
use rayon::prelude::*;

fn main() {
    let iterations = iterations();

    PROBLEMS.
        into_par_iter().
        for_each(|problem| {
            let reduced = Reducer::new(problem.clone()).reduce(iterations);
            println!("{}", reduced);
        });
}

fn iterations() -> usize {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<usize>().
        unwrap_or(1000)
}
