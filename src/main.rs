extern crate ukodus;

use ukodus::{PROBLEMS, Reducer};

fn main() {
    let iterations = iterations();

    let score = PROBLEMS.
        into_iter().
        map(|problem| {
            let problem = Reducer::new(problem.clone()).reduce(iterations);
            println!("{}", problem);
            problem.score()
        }).sum::<usize>();

    println!("Final Score: {}", score);
}

fn iterations() -> usize {
    std::env::var("ITERATIONS").
        unwrap_or(String::from("1000")).
        parse::<usize>().
        unwrap_or(1000)
}
