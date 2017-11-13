#![feature(test)]
extern crate ukodus;
extern crate test;

use ukodus::Puzzle;
use test::Bencher;

const SUDOKU: Puzzle = Puzzle([
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

#[bench]
fn bench_get(b: &mut Bencher) {
    b.iter(|| SUDOKU.get(8, 4))
}

#[bench]
fn bench_set(b: &mut Bencher) {
    let mut cloned = SUDOKU.clone();
    b.iter(|| cloned.set(8, 4, 0))
}

#[bench]
fn bench_rows(b: &mut Bencher) {
    b.iter(|| SUDOKU.rows())
}

#[bench]
fn bench_row(b: &mut Bencher) {
    b.iter(|| SUDOKU.row(5))
}

#[bench]
fn bench_cols(b: &mut Bencher) {
    b.iter(|| SUDOKU.cols())
}

#[bench]
fn bench_col(b: &mut Bencher) {
    b.iter(|| SUDOKU.col(5))
}

#[bench]
fn bench_boxes(b: &mut Bencher) {
    b.iter(|| SUDOKU.boxes())
}

#[bench]
fn bench_find_box(b: &mut Bencher) {
    b.iter(|| SUDOKU.find_box(4, 5))
}

#[bench]
fn bench_gaps(b: &mut Bencher) {
    b.iter(|| SUDOKU.gaps())
}

#[bench]
fn bench_is_valid(b: &mut Bencher) {
    b.iter(|| SUDOKU.is_valid())
}

const INCOMPLETE: Puzzle = Puzzle([
    8, 2, 6, 0, 9, 7, 5, 3, 1,
    1, 0, 4, 6, 3, 0, 2, 7, 9,
    7, 9, 3, 1, 2, 5, 4, 8, 6,
    4, 7, 0, 3, 6, 9, 8, 1, 2,
    2, 1, 8, 5, 7, 4, 6, 0, 3,
    3, 0, 9, 0, 8, 1, 7, 4, 5,
    5, 3, 7, 9, 4, 0, 1, 6, 8,
    9, 4, 2, 8, 1, 6, 3, 5, 7,
    6, 0, 1, 7, 5, 3, 9, 0, 4
]);

#[bench]
fn bench_gaps_incomplete(b: &mut Bencher) {
    b.iter(|| INCOMPLETE.gaps())
}

#[bench]
fn bench_is_valid_incomplete(b: &mut Bencher) {
    b.iter(|| INCOMPLETE.is_valid())
}

#[bench]
fn bench_possibilities_for_best_case(b: &mut Bencher) {
    let mut sudoku = SUDOKU.clone();
    sudoku.set(0, 0, 0);
    b.iter(|| sudoku.possibilities_for(0, 0))
}

#[bench]
fn bench_possibilities_for_worst_case(b: &mut Bencher) {
    let sudoku = Puzzle([
        0, 0, 0, 0, 0, 0, 0, 1, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5, 0, 4, 0, 7,
        0, 0, 8, 0, 0, 0, 3, 0, 0,
        0, 0, 1, 0, 9, 0, 0, 0, 0,
        3, 0, 0, 4, 0, 0, 2, 0, 0,
        0, 5, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 6, 0, 0, 0
    ]);

    b.iter(|| sudoku.possibilities_for(5, 5))
}
