// https://leetcode.com/problems/available-captures-for-rook/
// 999. Available Captures for Rook
pub struct Solution;
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut ri = 0;
        let mut rj = 0;
        'rook: for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'R' {
                    ri = i;
                    rj = j;
                    break 'rook;
                }
            }
        }
        let mut ans = 0;
        for j in (0..rj).rev() {
            if board[ri][j] == 'p' {
                ans += 1;
                break;
            } else if board[ri][j] == 'B' {
                break;
            }
        }
        for j in (rj + 1)..board[0].len() {
            if board[ri][j] == 'p' {
                ans += 1;
                break;
            } else if board[ri][j] == 'B' {
                break;
            }
        }
        for i in (0..ri).rev() {
            if board[i][rj] == 'p' {
                ans += 1;
                break;
            } else if board[i][rj] == 'B' {
                break;
            }
        }
        for i in (ri + 1)..board.len() {
            if board[i][rj] == 'p' {
                ans += 1;
                break;
            } else if board[i][rj] == 'B' {
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_rook_captures() {
        assert_eq!(
            Solution::num_rook_captures(vec_vec_chr![
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", "R", ".", ".", ".", "p"],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."]
            ]),
            3
        );
        assert_eq!(
            Solution::num_rook_captures(vec_vec_chr![
                [".", ".", ".", ".", ".", ".", "."],
                [".", "p", "p", "p", "p", "p", ".", "."],
                [".", "p", "p", "B", "p", "p", ".", "."],
                [".", "p", "B", "R", "B", "p", ".", "."],
                [".", "p", "p", "B", "p", "p", ".", "."],
                [".", "p", "p", "p", "p", "p", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."]
            ]),
            0
        );
        assert_eq!(
            Solution::num_rook_captures(vec_vec_chr![
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                ["p", "p", ".", "R", ".", "p", "B", "."],
                [".", ".", ".", ".", ".", ".", ".", "."],
                [".", ".", ".", "B", ".", ".", ".", "."],
                [".", ".", ".", "p", ".", ".", ".", "."],
                [".", ".", ".", ".", ".", ".", ".", "."]
            ]),
            3
        );
    }
}
