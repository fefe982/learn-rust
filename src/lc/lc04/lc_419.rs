// https://leetcode.com/problems/battleships-in-a-board/
// 419. Battleships in a Board
pub struct Solution;
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut cnt = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' && (i == 0 || board[i - 1][j] != 'X') && (j == 0 || board[i][j - 1] != 'X') {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_battleships() {
        assert_eq!(
            Solution::count_battleships(vec_vec_chr![
                ["X", ".", ".", "X"],
                [".", ".", ".", "X"],
                [".", ".", ".", "X"]
            ]),
            2
        );
        assert_eq!(Solution::count_battleships(vec_vec_chr![["."]]), 0)
    }
}
