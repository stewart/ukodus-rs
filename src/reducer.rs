use puzzle::Puzzle;
use rand::{thread_rng, Rng};

pub fn reduce(puzzle: &mut Puzzle) {
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
}
