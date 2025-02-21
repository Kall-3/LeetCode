use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut valid = true;

        for row in 0..9 {
            let row_vec: Vec<&char> = board[row].iter().filter(|x| **x != '.').collect();
            let row_set: HashSet<&&char> = row_vec.iter().collect();
            valid &= row_vec.len() == row_set.len();
        }

        for col in 0..9 {
            let col_vec: Vec<char> = board.iter().map(|x| x[col]).filter(|x| *x != '.').collect();
            let col_set: HashSet<&char> = col_vec.iter().collect();
            valid &= col_vec.len() == col_set.len();
        }

        for row in 0..3 {
            for col in 0..3 {
                let box_vec: Vec<&char> = board.iter().skip(row * 3).take(3).map(|x| {
                    x.iter().skip(col * 3).take(3)
                }).flatten().filter(|x| **x != '.').collect();
                let box_set: HashSet<&&char> = box_vec.iter().collect();
                valid &= box_vec.len() == box_set.len();
            }
        }

        valid
    }
}