// https://leetcode.com/problems/maximum-sum-of-an-hourglass/
// 2428. Maximum Sum of an Hourglass
pub struct Solution;
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                let sum = grid[i][j]
                    + grid[i][j + 1]
                    + grid[i][j + 2]
                    + grid[i + 1][j + 1]
                    + grid[i + 2][j]
                    + grid[i + 2][j + 1]
                    + grid[i + 2][j + 2];
                ans = ans.max(sum);
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
    fn max_sum() {
        assert_eq!(
            Solution::max_sum(vec_vec![[6, 2, 1, 3], [4, 2, 1, 5], [9, 2, 8, 7], [4, 1, 2, 9]]),
            30
        );
        assert_eq!(Solution::max_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]), 35);
    }
}
