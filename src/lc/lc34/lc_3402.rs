// https://leetcode.com/problems/minimum-operations-to-make-columns-strictly-increasing/
// 3402. Minimum Operations to Make Columns Strictly Increasing
pub struct Solution;
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid[0].len() {
            let mut pre = grid[0][i];
            for j in 1..grid.len() {
                if grid[j][i] <= pre {
                    ans += pre - grid[j][i] + 1;
                    pre = pre + 1;
                } else {
                    pre = grid[j][i];
                }
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
    fn minimum_operations() {
        assert_eq!(
            Solution::minimum_operations(vec_vec![[3, 2], [1, 3], [3, 4], [0, 1]]),
            15
        );
        assert_eq!(
            Solution::minimum_operations(vec_vec![[3, 2, 1], [2, 1, 0], [1, 2, 3]]),
            12
        );
    }
}
