// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
// 2482. Difference Between Ones and Zeros in Row and Column
pub struct Solution;
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut row = vec![0; m];
        let mut col = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                row[i] += grid[i][j] * 2 - 1;
                col[j] += grid[i][j] * 2 - 1;
            }
        }
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = row[i] + col[j];
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
    fn test_ones_minus_zeros() {
        assert_eq!(
            Solution::ones_minus_zeros(vec_vec![[0, 1, 1], [1, 0, 1], [0, 0, 1]]),
            vec_vec![[0, 0, 4], [0, 0, 4], [-2, -2, 2]]
        );
        assert_eq!(
            Solution::ones_minus_zeros(vec_vec![[1, 1, 1], [1, 1, 1]]),
            vec_vec![[5, 5, 5], [5, 5, 5]]
        );
    }
}
