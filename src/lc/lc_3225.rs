// https://leetcode.com/problems/maximum-score-from-grid-operations/
// 3225. Maximum Score From Performing Multiplication Operations
pub struct Solution;
impl Solution {
    fn max_score(grid: &Vec<Vec<i32>>, i: usize, mut pp: usize, p: usize, dp: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        if i >= grid[0].len() {
            return 0;
        }
        if pp <= p {
            pp = 0;
        }
        if dp[i][pp][p] != -1 {
            return dp[i][pp][p];
        }
        let mut add = 0;
        for j in 0..p {
            add += grid[j][i] as i64;
        }
        let mut ans = add + Self::max_score(grid, i + 1, p, 0, dp);
        for j in 0..p {
            add -= grid[j][i] as i64;
            if j + 1 >= grid.len() || grid[j + 1][i] > 0 {
                ans = ans.max(add + Self::max_score(grid, i + 1, p, j + 1, dp));
            }
        }
        for j in p..pp {
            if j + 1 >= grid.len() || grid[j + 1][i] > 0 {
                ans = ans.max(add + Self::max_score(grid, i + 1, 0, j + 1, dp));
            }
        }
        for j in pp.max(p)..grid.len() {
            add += grid[j][i - 1] as i64;
            if j + 1 >= grid.len() || grid[j + 1][i] > 0 {
                ans = ans.max(add + Self::max_score(grid, i + 1, 0, j + 1, dp));
            }
        }
        dp[i][pp][p] = ans;
        ans
    }
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        Self::max_score(
            &grid,
            0,
            grid.len(),
            0,
            &mut vec![vec![vec![-1; grid.len() + 1]; grid.len() + 1]; grid[0].len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_score() {
        assert_eq!(
            Solution::maximum_score(vec_vec![
                [0, 0, 0, 0, 0],
                [0, 0, 3, 0, 0],
                [0, 1, 0, 0, 0],
                [5, 0, 0, 3, 0],
                [0, 0, 0, 0, 2]
            ]),
            11
        );
        assert_eq!(
            Solution::maximum_score(vec_vec![
                [10, 9, 0, 0, 15],
                [7, 1, 0, 8, 0],
                [5, 20, 0, 11, 0],
                [0, 0, 0, 1, 2],
                [8, 12, 1, 10, 3]
            ]),
            94
        );
    }
}
