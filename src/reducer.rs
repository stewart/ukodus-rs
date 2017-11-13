use puzzle::Puzzle;
use rand::{thread_rng, Rng};

pub fn reduce(puzzle: &Puzzle, iterations: usize) -> Puzzle {
    let simple = simple(puzzle.clone());

    let rand = (0..iterations).
        map(|_| rand_simple(puzzle.clone())).
        max_by_key(|puzzle| puzzle.score()).
        unwrap();

    if simple.score() > rand.score() {
        return simple;
    } else {
        return rand;
    }
}

fn simple(mut puzzle: Puzzle) -> Puzzle {
    for (x, y) in puzzle.filled() {
        let current = puzzle.get(x, y);
        puzzle.set(x, y, 0);

        let possibilities = puzzle.possibilities_for(x, y).unwrap().len();

        if possibilities != 1 {
            puzzle.set(x, y, current);
        }
    }

    puzzle
}

fn rand_simple(mut puzzle: Puzzle) -> Puzzle {
    let mut rng = thread_rng();

    let mut filled = puzzle.filled();
    let filled = filled.as_mut_slice();
    rng.shuffle(filled);

    for &mut (x, y) in filled {
        let current = puzzle.get(x, y);
        puzzle.set(x, y, 0);

        let possibilities = puzzle.possibilities_for(x, y).unwrap().len();

        if possibilities != 1 {
            puzzle.set(x, y, current);
        }
    }

    puzzle
}
