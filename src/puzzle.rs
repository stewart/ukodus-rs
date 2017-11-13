use itertools::Itertools;

const SIZE: usize = 9;

const BOXES: [[usize; 9]; 9] = [
    [ 0,  1,  2,  9, 10, 11, 18, 19, 20],
    [ 3,  4,  5, 12, 13, 14, 21, 22, 23],
    [ 6,  7,  8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

pub struct Puzzle(pub [u8; 81]);

impl Puzzle {
    /// Creates a new Puzzle from the provided array
    pub fn new(puzzle: [u8; 81]) -> Puzzle {
        Puzzle(puzzle)
    }

    /// Gets a value from the specified column and row in the Puzzle
    pub fn get(&self, column: usize, row: usize) -> u8 {
        debug_assert!(column < 9);
        debug_assert!(row < 9);
        *self.0.get(row * SIZE + column).unwrap()
    }

    /// Returns a new Puzzle with the specified cell set to the provided value
    pub fn set(&self, column: usize, row: usize, value: u8) -> Puzzle {
        debug_assert!(column < 9);
        debug_assert!(row < 9);
        debug_assert!(value <= 9);

        let mut new_arr = (*self).0;
        new_arr[row * SIZE + column] = value;
        Puzzle(new_arr)
    }

    /// Returns all rows in the Puzzle
    pub fn rows(&self) -> Vec<&[u8]> {
        self.0.chunks(SIZE).collect()
    }

    /// Returns the specified row from the Puzzle
    pub fn row(&self, row: usize) -> &[u8] {
        debug_assert!(row < 9);
        self.0.chunks(SIZE).nth(row).unwrap()
    }

    /// Returns all columns in the Puzzle
    pub fn cols(&self) -> Vec<Vec<u8>> {
        (0..SIZE).map(|n| {
            self.0.iter().skip(n).step(SIZE).cloned().collect()
        }).collect()
    }

    /// Returns the specified column from the Puzzle
    pub fn col(&self, col: usize) -> Vec<u8> {
        debug_assert!(col < 9);
        self.0.iter().skip(col).step(SIZE).cloned().collect()
    }

    /// Returns all boxes (3x3 grids) in the Puzzle
    pub fn boxes(&self) -> Vec<Vec<u8>> {
        BOXES.iter().map(|b| {
            b.iter().map(|i| {
                self.0.get(*i).cloned().unwrap()
            }).collect()
        }).collect()
    }

    /// Returns the specified box from the Puzzle
    pub fn find_box(&self, column: usize, row: usize) -> Vec<u8> {
        debug_assert!(column < 9);
        debug_assert!(row < 9);

        let column = column / 3;
        let size = SIZE / 3;
        let row = row / 3;

        BOXES.get(row * size + column).unwrap().iter().map(|i| {
            self.0.get(*i).cloned().unwrap()
        }).collect()
    }

    /// Calculates the 'count' of the Puzzle, i.e. how many spots are filled
    pub fn count(&self) -> usize {
        self.0.iter().filter(|&i| i != &0).count()
    }

    /// Returns all possible values for the given cell coordinates
    pub fn possibilities_for(&self, column: usize, row: usize) -> Option<Vec<u8>> {
        if self.get(column, row) != 0 {
            return None;
        }

        let b = self.find_box(column, row);
        let row = self.row(row);
        let col = self.col(column);

        let possibilities = (1..10).filter(|i| {
             (!row.contains(i)) &&
             (!col.contains(i)) &&
             (!b.contains(i))
        }).collect();

        Some(possibilities)
    }

    /// Returns a vector of tuples indicating where gaps are in the Puzzle
    pub fn gaps(&self) -> Vec<(usize, usize)> {
        let mut gaps = vec![];

        for (idx, val) in self.0.iter().enumerate() {
            if val == &0 {
                gaps.push((idx % SIZE, idx / SIZE))
            }
        }

        gaps
    }

    /// Checks if the Puzzle is valid
    pub fn is_valid(&self) -> bool {
        if self.0.contains(&0) {
            return false;
        }

        let rows_valid = self.rows().iter().all(|row| {
            let initial_count = row.len();
            let mut row = row.to_vec();
            row.sort_unstable();
            row.dedup();

            row.len() == initial_count
        });

        if !rows_valid { return rows_valid; }

        let validate = |x: &mut Vec<u8>| -> bool {
            let initial = x.len();
            x.sort_unstable();
            x.dedup();
            x.len() == initial
        };

        self.cols().iter_mut().all(&validate) &&
        self.boxes().iter_mut().all(&validate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILLED: Puzzle = Puzzle([
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
    fn test_get() {
        assert_eq!(FILLED.get(0, 0), 8);
        assert_eq!(FILLED.get(3, 4), 5);
        assert_eq!(FILLED.get(5, 8), 3);
    }

    #[test]
    fn test_set() {
        assert_eq!(FILLED.get(0, 0), 8);

        let updated = FILLED.set(0, 0, 1);

        assert_eq!(FILLED.get(0, 0), 8);
        assert_eq!(updated.get(0, 0), 1);
    }

    #[test]
    fn test_rows() {
        assert_eq!(
            FILLED.rows(),
            [
                [8, 2, 6, 4, 9, 7, 5, 3, 1],
                [1, 5, 4, 6, 3, 8, 2, 7, 9],
                [7, 9, 3, 1, 2, 5, 4, 8, 6],
                [4, 7, 5, 3, 6, 9, 8, 1, 2],
                [2, 1, 8, 5, 7, 4, 6, 9, 3],
                [3, 6, 9, 2, 8, 1, 7, 4, 5],
                [5, 3, 7, 9, 4, 2, 1, 6, 8],
                [9, 4, 2, 8, 1, 6, 3, 5, 7],
                [6, 8, 1, 7, 5, 3, 9, 2, 4]
            ]
        )
    }

    #[test]
    fn test_row() {
        assert_eq!(FILLED.row(0), [8, 2, 6, 4, 9, 7, 5, 3, 1]);
        assert_eq!(FILLED.row(6), [5, 3, 7, 9, 4, 2, 1, 6, 8]);
    }

    #[test]
    fn test_cols() {
        assert_eq!(
            FILLED.cols(),
            [
                [8, 1, 7, 4, 2, 3, 5, 9, 6],
                [2, 5, 9, 7, 1, 6, 3, 4, 8],
                [6, 4, 3, 5, 8, 9, 7, 2, 1],
                [4, 6, 1, 3, 5, 2, 9, 8, 7],
                [9, 3, 2, 6, 7, 8, 4, 1, 5],
                [7, 8, 5, 9, 4, 1, 2, 6, 3],
                [5, 2, 4, 8, 6, 7, 1, 3, 9],
                [3, 7, 8, 1, 9, 4, 6, 5, 2],
                [1, 9, 6, 2, 3, 5, 8, 7, 4]
            ]
        )
    }

    #[test]
    fn test_col() {
        assert_eq!(FILLED.col(0), [8, 1, 7, 4, 2, 3, 5, 9, 6]);
        assert_eq!(FILLED.col(6), [5, 2, 4, 8, 6, 7, 1, 3, 9]);
    }

    #[test]
    fn test_boxes() {
        assert_eq!(
            FILLED.boxes(),
            [
                [8, 2, 6, 1, 5, 4, 7, 9, 3],
                [4, 9, 7, 6, 3, 8, 1, 2, 5],
                [5, 3, 1, 2, 7, 9, 4, 8, 6],
                [4, 7, 5, 2, 1, 8, 3, 6, 9],
                [3, 6, 9, 5, 7, 4, 2, 8, 1],
                [8, 1, 2, 6, 9, 3, 7, 4, 5],
                [5, 3, 7, 9, 4, 2, 6, 8, 1],
                [9, 4, 2, 8, 1, 6, 7, 5, 3],
                [1, 6, 8, 3, 5, 7, 9, 2, 4]
            ]
        )
    }

    #[test]
    fn test_find_box() {
        assert_eq!(FILLED.find_box(2, 2), [8, 2, 6, 1, 5, 4, 7, 9, 3]);
        assert_eq!(FILLED.find_box(4, 5), [3, 6, 9, 5, 7, 4, 2, 8, 1]);
        assert_eq!(FILLED.find_box(5, 0), [4, 9, 7, 6, 3, 8, 1, 2, 5]);
    }

    const INCOMPLETE: Puzzle = Puzzle([
        8, 2, 0, 4, 9, 7, 5, 3, 1,
        1, 5, 4, 0, 3, 8, 2, 7, 9,
        7, 9, 3, 1, 2, 5, 4, 8, 6,
        4, 7, 5, 0, 6, 9, 8, 1, 2,
        2, 1, 8, 5, 7, 4, 6, 9, 3,
        3, 6, 9, 2, 8, 1, 7, 4, 5,
        5, 3, 7, 9, 0, 2, 1, 6, 8,
        9, 4, 2, 8, 1, 6, 3, 5, 7,
        6, 8, 1, 7, 5, 3, 9, 2, 4
    ]);

    #[test]
    fn test_count() {
        assert_eq!(INCOMPLETE.count(), 77);
        assert_eq!(FILLED.count(), 81);
    }

    #[test]
    fn test_gaps() {
        assert_eq!(
            INCOMPLETE.gaps(),
            [(2, 0), (3, 1), (3, 3), (4, 6)]
        )
    }

    #[test]
    fn test_possibilities_for() {
        let sudoku = Puzzle::new([
            0, 2, 0, 4, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 8, 0, 0, 9,
            0, 9, 0, 0, 2, 0, 0, 0, 6,
            4, 7, 0, 0, 6, 9, 0, 1, 0,
            0, 0, 8, 5, 0, 0, 0, 0, 3,
            3, 0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 9, 4, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 5, 0,
            6, 0, 1, 0, 0, 3, 0, 2, 4
        ]);

        assert_eq!(sudoku.possibilities_for(0, 0), Some(vec![1, 5, 7, 8]));
        assert_eq!(sudoku.possibilities_for(5, 0), Some(vec![5, 6, 7]));
        assert_eq!(sudoku.possibilities_for(8, 7), Some(vec![1, 7, 8]));
    }

    #[test]
    fn test_is_valid() {
        assert!(FILLED.is_valid());
        assert!(!INCOMPLETE.is_valid());

        let with_dup = FILLED.set(0, 0, 1);
        assert!(!with_dup.is_valid());
    }
}
