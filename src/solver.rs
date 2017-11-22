use puzzle::Puzzle;

pub struct Solver {
    puzzle: Puzzle
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Solver {
        Solver { puzzle: puzzle }
    }

    pub fn solve(&self) -> Puzzle {
        let mut best = self.puzzle.clone();

        loop {
            let current = fill_in_known(best.clone());

            if current.count() > best.count() {
                best = current;
            } else {
                break;
            }
        }

        best
    }

    pub fn is_solvable(&self) -> bool {
        self.solve().score() == 0
    }
}

fn fill_in_known(mut puzzle: Puzzle) -> Puzzle {
    for (col, row) in puzzle.gaps() {
        let possibilities = puzzle.possibilities_for(col, row).unwrap();

        if possibilities.len() == 1 {
            let value = possibilities.first().unwrap();
            puzzle.set(col, row, *value);
        }
    }

    puzzle
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzle::Puzzle;

    const SOLVED: Puzzle = Puzzle([
        8, 2, 6, 4, 9, 7, 5, 3, 1,
        1, 5, 4, 6, 3, 8, 2, 7, 9,
        7, 9, 3, 1, 2, 5, 4, 8, 6,
        4, 7, 5, 3, 6, 9, 8, 1, 2,
        2, 1, 8, 5, 7, 4, 6, 9, 3,
        3, 6, 9, 2, 8, 1, 7, 4, 5,
        5, 3, 7, 9, 4, 2, 1, 6, 8,
        9, 4, 2, 8, 1, 6, 3, 5, 7,
        6, 8, 1, 7, 5, 3, 9, 2, 4
    ]);

    #[test]
    fn test_solver_basic() {
        let mut unsolved = SOLVED.clone();
        unsolved.set(0, 0, 0);

        let solver = Solver::new(unsolved);
        assert_eq!(solver.solve(), SOLVED);
    }

    #[test]
    fn test_solver_hard() {
        let solver = Solver::new(
            Puzzle([
                   0, 0, 0, 0, 0, 8, 0, 0, 1,
                   4, 0, 0, 0, 2, 0, 3, 0, 0,
                   9, 0, 2, 5, 0, 0, 0, 6, 0,
                   0, 0, 0, 0, 3, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 6, 0, 0, 0,
                   5, 0, 0, 0, 0, 0, 8, 0, 9,
                   0, 8, 0, 0, 0, 2, 0, 0, 0,
                   0, 2, 0, 0, 6, 0, 7, 0, 0,
                   0, 0, 0, 0, 7, 0, 0, 3, 0
            ])
        );

        assert_eq!(
            solver.solve(),
            Puzzle([
                6, 5, 3, 4, 9, 8, 2, 7, 1,
                4, 1, 8, 6, 2, 7, 3, 9, 5,
                9, 7, 2, 5, 1, 3, 4, 6, 8,
                8, 9, 1, 2, 3, 5, 6, 4, 7,
                2, 4, 7, 9, 8, 6, 1, 5, 3,
                5, 3, 6, 7, 4, 1, 8, 2, 9,
                7, 8, 4, 3, 5, 2, 9, 1, 6,
                3, 2, 5, 1, 6, 9, 7, 8, 4,
                1, 6, 9, 8, 7, 4, 5, 3, 2
            ])
        )
    }

    #[test]
    fn test_solvable() {
        let mut puzzle = Puzzle([
            0, 0, 0, 0, 0, 8, 0, 0, 1,
            4, 0, 0, 0, 2, 0, 3, 0, 0,
            9, 0, 2, 5, 0, 0, 0, 6, 0,
            0, 0, 0, 0, 3, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 6, 0, 0, 0,
            5, 0, 0, 0, 0, 0, 8, 0, 9,
            0, 8, 0, 0, 0, 2, 0, 0, 0,
            0, 2, 0, 0, 6, 0, 7, 0, 0,
            0, 0, 0, 0, 7, 0, 0, 3, 0
        ]);

        let solver = Solver::new(puzzle.clone());
        assert!(solver.is_solvable());

        puzzle.set(0, 1, 0);
        let solver = Solver::new(puzzle.clone());
        assert!(!solver.is_solvable());
    }
}
