// https://leetcode.com/problems/maximal-square/
// 221. Maximal Square
pub struct Solution;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        let mut max = 0;
        for i in 1..=matrix.len() {
            for j in 1..=matrix[0].len() {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]) + 1;
                    max = max.max(dp[i][j])
                }
            }
        }
        max * max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximal_square() {
        assert_eq!(
            Solution::maximal_square(vec_vec_chr![
                ["1", "0", "1", "0", "0"],
                ["1", "0", "1", "1", "1"],
                ["1", "1", "1", "1", "1"],
                ["1", "0", "0", "1", "0"]
            ]),
            4
        );
        assert_eq!(Solution::maximal_square(vec_vec_chr![["0", "1"], ["1", "0"]]), 1);
        assert_eq!(Solution::maximal_square(vec_vec_chr![["0"]]), 0);
    }
}
