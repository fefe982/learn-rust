// https://leetcode.com/problems/pizza-with-3n-slices/
// 1388. Pizza With 3n Slices
pub struct Solution;
impl Solution {
    fn max_slices(v: &[i32]) -> i32 {
        let n = v.len();
        let l = (n + 1) / 3;
        let mut dp = vec![vec![i32::MIN; l + 1]; n];
        dp[0][0] = 0;
        dp[0][1] = v[0];
        dp[1][0] = 0;
        dp[1][1] = v[0].max(v[1]);
        for i in 2..n {
            dp[i][0] = 0;
            for j in 1..l + 1 {
                dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + v[i]);
            }
        }
        dp[n - 1][l]
    }
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        return Self::max_slices(&slices[1..]).max(Self::max_slices(&slices[..slices.len() - 1]));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_size_slices() {
        assert_eq!(
            Solution::max_size_slices(vec![
                2, 4, 3, 1, 5, 9, 4, 4, 5, 6, 5, 7, 6, 3, 10, 6, 8, 8, 8, 3, 5, 5, 8, 1, 8, 2, 6,
                7, 2, 1, 3, 4, 7, 10, 8, 7, 9, 5, 1, 1, 9, 10, 3, 10, 5, 10, 5, 2, 4, 6, 6, 1, 9,
                4, 8, 2, 1
            ]),
            153
        );
        assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    }
}
