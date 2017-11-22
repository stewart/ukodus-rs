use puzzle::Puzzle;
use solver::Solver;
use rand::{thread_rng, Rng};

use rayon::prelude::*;

pub struct Reducer {
    puzzle: Puzzle,
}

impl Reducer {
    pub fn new(puzzle: Puzzle) -> Reducer {
        Reducer { puzzle: puzzle }
    }

    pub fn reduce(&self, iterations: usize) -> Puzzle {
        let filled = self.puzzle.filled();
        let mut rng = thread_rng();

        let iterations = (0..iterations).map(|_| {
            let mut filled = filled.clone();
            rng.shuffle(&mut filled);
            filled
        }).collect::<Vec<_>>();

        iterations.into_par_iter().map(|filled| {
            let mut best = self.puzzle.clone();

            for (x, y) in filled {
                let mut attempt = best.clone();
                attempt.set(x, y, 0);

                if Solver::new(attempt.clone()).is_solvable() {
                    best = attempt;
                }
            }
            best
        }).max_by_key(|puzzle| puzzle.score()).unwrap()
    }
}
