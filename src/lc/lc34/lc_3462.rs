// https://leetcode.com/problems/maximum-sum-with-at-most-k-elements/
// 3462. Maximum Sum With at Most K Elements
pub struct Solution;
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        let mut v = vec![];
        let mut grid = grid;
        for i in 0..grid.len() {
            grid[i].sort_unstable_by_key(|x| -x);
            for j in 0..limits[i] as usize {
                v.push(grid[i][j]);
            }
        }
        v.sort_unstable_by_key(|x| -x);
        let mut sum = 0;
        for i in 0..k as usize {
            sum += v[i] as i64;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_sum() {
        assert_eq!(Solution::max_sum(vec_vec![[1, 2], [3, 4]], vec![1, 2], 2), 7);
        assert_eq!(Solution::max_sum(vec_vec![[5, 3, 7], [8, 2, 6]], vec![2, 2], 3), 21);
    }
}
