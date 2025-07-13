// https://leetcode.com/problems/minimum-falling-path-sum-ii/
// 1289. Minimum Falling Path Sum II
pub struct Solution;
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut min0 = i32::MAX;
        let mut idx0 = 0;
        let mut min1 = i32::MAX;
        for i in 0..grid[0].len() {
            if grid[0][i] < min0 {
                min1 = min0;
                idx0 = i;
                min0 = grid[0][i];
            } else if grid[0][i] < min1 {
                min1 = grid[0][i];
            }
        }
        for i in 1..grid.len() {
            let mut m0 = i32::MAX;
            let mut i0 = 0;
            let mut m1 = i32::MAX;
            for j in 0..grid[i].len() {
                let s = grid[i][j] + if j == idx0 { min1 } else { min0 };
                if s < m0 {
                    m1 = m0;
                    i0 = j;
                    m0 = s;
                } else if s < m1 {
                    m1 = s;
                }
            }
            min0 = m0;
            idx0 = i0;
            min1 = m1;
        }
        min0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_falling_path_sum() {
        assert_eq!(
            Solution::min_falling_path_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            13
        );
        assert_eq!(Solution::min_falling_path_sum(vec_vec![[1]]), 1);
    }
}
