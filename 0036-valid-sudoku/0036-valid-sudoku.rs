use std::collections::HashSet;
use std::any::type_name;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {       // Check if a row contains dup
                if board[row].iter().enumerate().any(|(idx, elem)| elem != &'.' && idx != col && elem == &board[row][col])
                {
                    return false;
                }
           }
        }
        for col in 0..9 {
            for row in 0..9 {
                if board.iter().map(|x| x[col]).enumerate().any(|(idx, elem)| elem != '.' && idx != row && &elem == &board[row][col])
                {
                    return false;
                }

            }
        }
        for row in 0..9 {
            let b_r = row / 3;
            for col in 0..9 {
                let b_c = col / 3;
                // println!("box: {}. b_r, b_c: ({}, {})", boxes, b_r, b_c);
                if board.iter().enumerate()
                    .skip(b_r * 3).take(3)
                    .flat_map(|(r, v)| v.iter()
                        .enumerate()
                        .skip(b_c * 3).take(3)
                        .filter(move |&(c, _)| r != row || c != col)
                        .map(move |(c, &elem)| (r, c, elem)))
                    .any(|(r, c, elem)| elem != '.' && &elem == &board[row][col])
                {
                    return false;
                }
            }
        }
        
        return true;
    }
}