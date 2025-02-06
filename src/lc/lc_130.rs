// https://leetcode.com/problems/surrounded-regions/
// 130. Surrounded Regions
pub struct Solution;
impl Solution {
    fn fill(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let mut q = vec![(i, j)];
        board[i][j] = 'B';
        while let Some((i, j)) = q.pop() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = ((i as isize + dx) as usize, (j as isize + dy) as usize);
                if ni < board.len() && nj < board[0].len() && board[ni][nj] == 'O' {
                    board[ni][nj] = 'B';
                    q.push((ni, nj));
                }
            }
        }
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in 0..board.len() {
            if board[i][0] == 'O' {
                Self::fill(board, i, 0);
            }
            if board[i][board[0].len() - 1] == 'O' {
                Self::fill(board, i, board[0].len() - 1);
            }
        }
        for j in 0..board[0].len() {
            if board[0][j] == 'O' {
                Self::fill(board, 0, j);
            }
            if board[board.len() - 1][j] == 'O' {
                Self::fill(board, board.len() - 1, j);
            }
        }
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'B' {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(board: &mut Vec<Vec<char>>, expected: Vec<Vec<char>>) {
        Solution::solve(board);
        assert_eq!(board, &expected);
    }
    #[test]
    fn test_solve() {
        check(
            &mut vec_vec_chr![
                ["X", "X", "X", "X"],
                ["X", "O", "O", "X"],
                ["X", "X", "O", "X"],
                ["X", "O", "X", "X"]
            ],
            vec_vec_chr![
                ["X", "X", "X", "X"],
                ["X", "X", "X", "X"],
                ["X", "X", "X", "X"],
                ["X", "O", "X", "X"]
            ],
        );
        check(&mut vec_vec_chr![["X"]], vec_vec_chr![["X"]]);
    }
}
