// https://leetcode.com/problems/sort-matrix-by-diagonals/
// 3446. Sort Matrix by Diagonals
pub struct Solution;
impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut grid = grid;
        for i in 0..n {
            let mut v1 = vec![];
            let mut v2 = vec![];
            for j in 0..n - i {
                v1.push(grid[j + i][j]);
                if i != 0 {
                    v2.push(grid[j][j + i]);
                }
            }
            v1.sort_by_key(|&x| -x);
            if i != 0 {
                v2.sort();
            }
            for j in 0..n - i {
                grid[j + i][j] = v1[j];
                if i != 0 {
                    grid[j][j + i] = v2[j];
                }
            }
        }
        grid
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sort_matrix() {
        assert_eq!(
            Solution::sort_matrix(vec_vec![[1, 7, 3], [9, 8, 2], [4, 5, 6]]),
            vec_vec![[8, 2, 3], [9, 6, 7], [4, 5, 1]]
        );
        assert_eq!(
            Solution::sort_matrix(vec_vec![[0, 1], [1, 2]]),
            vec_vec![[2, 1], [1, 0]]
        );
        assert_eq!(Solution::sort_matrix(vec_vec![[1]]), vec_vec![[1]]);
    }
}
