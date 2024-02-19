// https://leetcode.com/problems/number-of-paths-with-max-score/
// 1301. Number of Paths with Max Score
pub struct Solution;
impl Solution {
    fn update(base: (i32, i64), add: (i32, i64)) -> (i32, i64) {
        if base.0 < add.0 {
            add
        } else if base.0 > add.0 {
            base
        } else {
            (base.0, (base.1 + add.1) % 1_0000_0000_7)
        }
    }
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let board = board
            .into_iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut dp = vec![vec![(0, 0); board[0].len()]; board.len()];
        dp[board.len() - 1][board[0].len() - 1] = (0, 1);
        for i in (0..board.len()).rev() {
            for j in (0..board[0].len()).rev() {
                if i == board.len() - 1 && j == board[0].len() - 1 {
                    continue;
                }
                if board[i][j] == 'X' {
                    dp[i][j] = (0, 0);
                } else {
                    let mut cnt = (0, 0);
                    if i + 1 < board.len() {
                        cnt = Self::update(cnt, dp[i + 1][j]);
                    }
                    if (j + 1) < board[0].len() {
                        cnt = Self::update(cnt, dp[i][j + 1]);
                    }
                    if (i + 1) < board.len() && (j + 1) < board[0].len() {
                        cnt = Self::update(cnt, dp[i + 1][j + 1]);
                    }
                    if i == 0 && j == 0 {
                        return vec![cnt.0, cnt.1 as i32];
                    }
                    if cnt.1 == 0 {
                        dp[i][j] = cnt;
                    } else {
                        dp[i][j] = (cnt.0 + (board[i][j] as u8 - '0' as u8) as i32, cnt.1)
                    }
                }
            }
        }
        vec![dp[0][0].0, dp[0][0].1 as i32]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_paths_with_max_score() {
        assert_eq!(
            Solution::paths_with_max_score(vec_str!["E23", "2X2", "12S"]),
            vec![7, 1]
        );
        assert_eq!(
            Solution::paths_with_max_score(vec_str!["E12", "1X1", "21S"]),
            vec![4, 2]
        );
        assert_eq!(
            Solution::paths_with_max_score(vec_str!["E11", "XXX", "11S"]),
            vec![0, 0]
        );
    }
}
