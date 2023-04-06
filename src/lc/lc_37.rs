// https://leetcode.com/problems/sudoku-solver/description/
// 37. Sudoku Solver
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
    fn check_board(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let mut check = vec![false; 9];
        for i in 0..9 {
            if !Self::check_pos(board[x][i], &mut check) {
                return false;
            }
        }
        check.fill(false);
        for i in 0..9 {
            if !Self::check_pos(board[i][y], &mut check) {
                return false;
            }
        }
        check.fill(false);
        let sx = x / 3;
        let sy = y / 3;
        for ix in 0..3 {
            for iy in 0..3 {
                if !Self::check_pos(board[sx * 3 + ix][sy * 3 + iy], &mut check) {
                    return false;
                }
            }
        }
        true
    }
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] == '.' {
                    for c in 1..=9 {
                        board[x][y] = char::from_digit(c, 10).unwrap();
                        if Self::check_board(board, x, y) && Self::solve(board) {
                            return true;
                        }
                    }
                    board[x][y] = '.';
                    return false;
                }
            }
        }
        true
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_board(board: [[&str; 9]; 9]) -> Vec<Vec<char>> {
        let mut res = vec![vec!['\0'; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                res[i][j] = board[i][j].chars().nth(0).unwrap();
            }
        }
        res
    }
    #[test]
    fn my_pow() {
        let mut board = make_board([
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]);
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            make_board([
                ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
                ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
                ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
                ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
                ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
                ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
                ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
                ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
                ["3", "4", "5", "2", "8", "6", "1", "7", "9"]
            ])
        );
    }
}
