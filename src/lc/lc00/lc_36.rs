// https://leetcode.com/problems/valid-sudoku/
// 36. Valid Sudoku
pub struct Solution;
impl Solution {
    fn check_pos(c: char, check: &mut Vec<bool>) -> bool {
        if c != '.' {
            let digit = c.to_digit(10).unwrap() as usize - 1;
            if check[digit] {
                return false;
            }
            check[digit] = true
        }
        true
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut check = vec![false; 9];
        for x in 0..9 {
            check.fill(false);
            for i in 0..9 {
                if !Self::check_pos(board[x][i], &mut check) {
                    return false;
                }
            }
        }
        for y in 0..9 {
            check.fill(false);
            for i in 0..9 {
                if !Self::check_pos(board[i][y], &mut check) {
                    return false;
                }
            }
        }
        for sx in 0..3 {
            for sy in 0..3 {
                check.fill(false);
                for ix in 0..3 {
                    for iy in 0..3 {
                        if !Self::check_pos(board[sx * 3 + ix][sy * 3 + iy], &mut check) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_valid_sudoku() {
        assert_eq!(
            Solution::is_valid_sudoku(vec_vec_chr![
                ["5", "3", ".", ".", "7", ".", ".", ".", "."],
                ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                [".", "9", "8", ".", ".", ".", ".", "6", "."],
                ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                [".", "6", ".", ".", ".", ".", "2", "8", "."],
                [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                [".", ".", ".", ".", "8", ".", ".", "7", "9"]
            ]),
            true
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec_vec_chr![
                ["8", "3", ".", ".", "7", ".", ".", ".", "."],
                ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                [".", "9", "8", ".", ".", ".", ".", "6", "."],
                ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                [".", "6", ".", ".", ".", ".", "2", "8", "."],
                [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                [".", ".", ".", ".", "8", ".", ".", "7", "9"]
            ]),
            false
        );
    }
}
