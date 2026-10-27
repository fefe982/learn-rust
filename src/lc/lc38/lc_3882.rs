// https://leetcode.com/problems/minimum-xor-path-in-a-grid/
// 3882. Minimum XOR Path in a Grid
pub struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![std::collections::BTreeSet::new(); grid[0].len()];
        for i in 0..grid[0].len() {
            if i == 0 {
                dp[0].insert(grid[0][0]);
            } else {
                dp[i] = dp[i - 1].iter().map(|x| x ^ grid[0][i]).collect();
            }
        }
        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                if j == 0 {
                    dp[0] = dp[0].iter().map(|x| x ^ grid[i][j]).collect();
                } else {
                    dp[j] = dp[j - 1].union(&dp[j]).map(|x| x ^ grid[i][j]).collect();
                }
            }
        }
        *dp[grid[0].len() - 1].iter().next().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec_vec![[1, 2], [3, 4]]), 6);
        assert_eq!(Solution::min_cost(vec_vec![[6, 7], [5, 8]]), 9);
        assert_eq!(Solution::min_cost(vec_vec![[2, 7, 5]]), 0);
    }
}
