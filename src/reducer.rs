use puzzle::Puzzle;
use rand::{thread_rng, Rng};

type Coordinates = (usize, usize);

pub fn reduce(puzzle: &Puzzle, iterations: usize) -> Puzzle {
    let mut puzzles = vec![];

    puzzles.push(simple_reducer(puzzle.clone(), puzzle.filled()));

    for _ in 0..iterations {
        puzzles.push(simple_reducer(puzzle.clone(), random_filled(&puzzle)));
    }

    puzzles.into_iter().max_by_key(|puzzle| puzzle.score()).unwrap()
}

// Given a Puzzle and a Vec of coordinates (col, row), attempts to remove as
// many cells as possible, while ensuring the Puzzle is still solvable.
fn simple_reducer(mut puzzle: Puzzle, cells: Vec<Coordinates>) -> Puzzle {
    for (col, row) in cells {
        let current = puzzle.get(col, row);
        puzzle.set(col, row, 0);

        let possibilities = puzzle.possibilities_for(col, row).unwrap().len();

        if possibilities != 1 {
            puzzle.set(col, row, current);
        }
    }

    puzzle
}

fn random_filled(puzzle: &Puzzle) -> Vec<Coordinates> {
    let mut rng = thread_rng();
    let mut filled = puzzle.filled();
    rng.shuffle(&mut filled);
    filled
}
