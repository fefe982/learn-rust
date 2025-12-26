// https://leetcode.com/problems/find-missing-and-repeated-values/
// 2965. Find Missing and Repeated Values
pub struct Solution;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = 0;
        let mut sum_sq = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                sum += grid[i][j] as i64;
                sum_sq += grid[i][j] as i64 * grid[i][j] as i64;
            }
        }
        let n = grid.len() as i64 * grid.len() as i64;
        let d1 = (n * (n + 1) / 2) - sum;
        let d2 = (n * (n + 1) * (2 * n + 1) / 6) - sum_sq;
        vec![((d2 / d1 - d1) / 2) as i32, ((d1 + d2 / d1) / 2) as i32]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_missing_and_repeated_values() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec_vec![[1, 3], [2, 2]]),
            [2, 4]
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec_vec![[9, 1, 7], [8, 9, 2], [3, 4, 6]]),
            [9, 5]
        );
    }
}
