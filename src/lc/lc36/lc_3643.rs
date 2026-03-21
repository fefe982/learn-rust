// https://leetcode.cn/problems/flip-square-submatrix-vertically/
// 3643. Flip Square Submatrix Vertically
pub struct Solution;
impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        for i in x..x + k / 2 {
            for j in 0..k {
                let tmp = grid[i][y + j];
                grid[i][y + j] = grid[x + k - 1 - (i - x)][y + j];
                grid[x + k - 1 - (i - x)][y + j] = tmp;
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
    fn reverse_submatrix() {
        assert_eq!(
            Solution::reverse_submatrix(
                vec_vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]],
                1,
                0,
                3
            ),
            vec_vec![[1, 2, 3, 4], [13, 14, 15, 8], [9, 10, 11, 12], [5, 6, 7, 16]]
        );
        assert_eq!(
            Solution::reverse_submatrix(vec_vec![[3, 4, 2, 3], [2, 3, 4, 2]], 0, 2, 2),
            vec_vec![[3, 4, 4, 2], [2, 3, 2, 3]]
        );
    }
}
