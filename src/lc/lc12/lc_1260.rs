// https://leetcode.com/problems/shift-2d-grid/description/
// 1260. Shift 2D Grid
pub struct Solution;
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];
        let k = k as usize;
        for i in 0..m {
            for j in 0..n {
                let x = (i * n + j + k as usize) % (m * n);
                ans[x / n][x % n] = grid[i][j];
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
    fn shift_grid() {
        assert_eq!(
            Solution::shift_grid(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 1),
            vec_vec![[9, 1, 2], [3, 4, 5], [6, 7, 8]]
        );
        assert_eq!(
            Solution::shift_grid(
                vec_vec![[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]],
                4
            ),
            vec_vec![[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
        );
        assert_eq!(
            Solution::shift_grid(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 9),
            vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        );
    }
}
