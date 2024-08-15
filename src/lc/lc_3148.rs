// https://leetcode.com/problems/maximum-difference-score-in-a-grid/
// 3148. Maximum Difference Score in a Grid
pub struct Solution;
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max = i32::MIN;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut min = i32::MAX;
                if i > 0 {
                    min = min.min(grid[i - 1][j]);
                }
                if j > 0 {
                    min = min.min(grid[i][j - 1]);
                }
                if min != i32::MAX {
                    max = max.max(grid[i][j] - min);
                }
                grid[i][j] = grid[i][j].min(min);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_score() {
        assert_eq!(
            Solution::max_score(vec_vec![[9, 5, 7, 3], [8, 9, 6, 1], [6, 7, 14, 3], [2, 5, 3, 1]]),
            9
        );
        assert_eq!(Solution::max_score(vec_vec![[4, 3, 2], [3, 2, 1]]), -1);
    }
}
