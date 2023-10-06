use serde::{Deserialize, Serialize};

const SIZE: usize = 9;

#[derive(Serialize, Deserialize)]
pub struct Sudoku {
    board: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    pub fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut is_empty = false;
    
        // find empty cell
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    row = i;
                    col = j;
                    is_empty = true;
                    break;
                }
            }
            if is_empty {
                break;
            }
        }
    
        if !is_empty {
            return true;
        }
    
        // test all values for this cell
        for num in 1..=SIZE {
            if self.is_safe(row, col, num as u8) {
                self.board[row][col] = num as u8;
                
                // recursion happens here
                if self.solve() {
                    return true;
                }
                // if it was unable to find a solution, change this cell back to 0
                self.board[row][col] = 0;
            }
        }
    
        // if there was no valid answer
        false
    }
    
    fn is_safe(&self, row: usize, col: usize, num:u8) -> bool {
        // checks if the same number already exists in the same row/col
        for i in 0..SIZE {
            if self.board[row][i] == num {
                return false;
            }
            if self.board[i][col] == num {
                return false;
            }
        }
    
        // checks if the same number already exists in this square
        let start_row = row - row % 3;
        let start_col = col - col % 3;
    
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j+start_col] == num {
                    return false;
                }
            }
        }
    
        true
    }
}