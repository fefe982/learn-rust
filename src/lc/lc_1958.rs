// https://leetcode.com/problems/check-if-move-is-legal/
// 1958. Check if Move is Legal
pub struct Solution;
impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let (r, c) = (r_move as usize, c_move as usize);
        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            for i in 1.. {
                let (nr, nc) = ((r as i32 + dir.0 * i) as usize, (c as i32 + dir.1 * i) as usize);
                if nr >= board.len() || nc >= board[0].len() {
                    break;
                }
                if board[nr][nc] == '.' {
                    break;
                }
                if board[nr][nc] == color {
                    if i > 1 {
                        return true;
                    }
                    break;
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_move() {
        assert_eq!(
            Solution::check_move(
                vec_vec_chr![
                    [".", ".", ".", "B", ".", ".", ".", "."],
                    [".", ".", ".", "W", ".", ".", ".", "."],
                    [".", ".", ".", "W", ".", ".", ".", "."],
                    [".", ".", ".", "W", ".", ".", ".", "."],
                    ["W", "B", "B", ".", "W", "W", "W", "B"],
                    [".", ".", ".", "B", ".", ".", ".", "."],
                    [".", ".", ".", "B", ".", ".", ".", "."],
                    [".", ".", ".", "W", ".", ".", ".", "."]
                ],
                4,
                3,
                'B'
            ),
            true
        );
        assert_eq!(
            Solution::check_move(
                vec_vec_chr![
                    [".", ".", ".", ".", ".", ".", ".", "."],
                    [".", "B", ".", ".", "W", ".", ".", "."],
                    [".", ".", "W", ".", ".", ".", ".", "."],
                    [".", ".", ".", "W", "B", ".", ".", "."],
                    [".", ".", ".", ".", ".", ".", ".", "."],
                    [".", ".", ".", ".", "B", "W", ".", "."],
                    [".", ".", ".", ".", ".", ".", "W", "."],
                    [".", ".", ".", ".", ".", ".", ".", "B"]
                ],
                4,
                4,
                'W'
            ),
            false
        );
    }
}
