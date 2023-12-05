// https://leetcode.com/problems/transform-to-chessboard/
// 782. Transform to Chessboard
pub struct Solution;
impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut cnt = 0;
        let mut c0: i32 = 0;
        let mut c1: i32 = 0;
        let mut misplace = 0;
        for i in 0..n {
            if board[0][i] == 0 {
                c0 += 1;
            } else {
                c1 += 1;
            }
            if board[0][i] != i as i32 % 2 {
                misplace += 1;
            }
        }
        if (c0 - c1).abs() > 1 {
            return -1;
        }
        cnt += if c1 > c0 {
            n as i32 - misplace
        } else if c1 == c0 {
            misplace.min(n as i32 - misplace)
        } else {
            misplace
        } / 2;
        if board[0][0] == 0 {
            c0 = 1;
            c1 = 0;
            misplace = 0;
        } else {
            c1 = 1;
            c0 = 0;
            misplace = 1;
        }
        for i in 1..n {
            if board[i][0] == 0 {
                c0 += 1;
            } else {
                c1 += 1;
            }
            if board[i][0] != i as i32 % 2 {
                misplace += 1;
            }
            for j in 1..n {
                if (board[i][j] - board[0][j]).abs() != (board[i][0] - board[0][0]).abs() {
                    return -1;
                }
            }
        }
        if (c0 - c1).abs() > 1 {
            return -1;
        }
        cnt += if c1 > c0 {
            n as i32 - misplace
        } else if c1 == c0 {
            misplace.min(n as i32 - misplace)
        } else {
            misplace
        } / 2;
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_moves_to_chessboard() {
        assert_eq!(
            Solution::moves_to_chessboard(vec_vec![[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]]),
            2
        );
        assert_eq!(Solution::moves_to_chessboard(vec_vec![[0, 1], [1, 0]]), 0);
        assert_eq!(Solution::moves_to_chessboard(vec_vec![[1, 0], [1, 0]]), -1);
    }
}
