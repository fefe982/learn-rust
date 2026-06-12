// https://leetcode.com/problems/check-if-matrix-is-x-matrix/
// 2319. Check if Matrix Is X-Matrix
pub struct Solution;
impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                if (i == j || i + j == n - 1) && grid[i][j] == 0 {
                    return false;
                }
                if (i != j && i + j != n - 1) && grid[i][j] != 0 {
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
    fn test_check_x_matrix() {
        assert_eq!(
            Solution::check_x_matrix(vec_vec![[2, 0, 0, 1], [0, 3, 1, 0], [0, 5, 2, 0], [4, 0, 0, 2]]),
            true
        );
        assert_eq!(
            Solution::check_x_matrix(vec_vec![[5, 7, 0], [0, 3, 1], [0, 5, 0]]),
            false
        );
    }
}
