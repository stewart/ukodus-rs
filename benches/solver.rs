#![feature(test)]
extern crate ukodus;
extern crate test;

use ukodus::{Puzzle, solve};
use test::Bencher;

#[bench]
fn bench_solve_easy(b: &mut Bencher) {
    b.iter(|| {
        let sudoku = Puzzle([
            0, 2, 6, 4, 9, 7, 5, 3, 1,
            1, 5, 4, 6, 3, 8, 2, 7, 9,
            7, 9, 3, 1, 2, 5, 4, 8, 6,
            4, 7, 5, 3, 6, 9, 8, 1, 2,
            2, 1, 8, 5, 7, 4, 6, 9, 3,
            3, 6, 9, 2, 8, 1, 7, 4, 5,
            5, 3, 7, 9, 4, 2, 1, 6, 8,
            9, 4, 2, 8, 1, 6, 3, 5, 7,
            6, 8, 1, 7, 5, 3, 9, 2, 4
        ]);

        solve(sudoku)
    })
}

#[bench]
fn bench_solve_hard(b: &mut Bencher) {
    b.iter(|| {
        let sudoku = Puzzle([
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

        solve(sudoku)
    })
}
