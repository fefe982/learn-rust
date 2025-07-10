// https://leetcode.com/problems/check-if-grid-satisfies-conditions/
// 3142. Check if Grid Satisfies Conditions
pub struct Solution;
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for j in 1..grid[0].len() {
            if grid[0][j] == grid[0][j - 1] {
                return false;
            }
        }
        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != grid[i - 1][j] {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_if_grid_satisfies_conditions() {
        assert_eq!(Solution::satisfies_conditions(vec_vec![[1, 0, 2], [1, 0, 2]]), true);
        assert_eq!(Solution::satisfies_conditions(vec_vec![[1, 1, 1], [0, 0, 0]]), false);
        assert_eq!(Solution::satisfies_conditions(vec_vec![[1], [2], [3]]), false);
    }
}
