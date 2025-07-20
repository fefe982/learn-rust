// https://leetcode.com/problems/surface-area-of-3d-shapes/
// 892. Surface Area of 3D Shapes
pub struct Solution;
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        for i in 0..n {
            ans += grid[i][0] + grid[0][i] + grid[i][n - 1] + grid[n - 1][i];
            for j in 0..n {
                if grid[i][j] > 0 {
                    ans += 2;
                }
                if i > 0 {
                    ans += (grid[i][j] - grid[i - 1][j]).abs();
                }
                if j > 0 {
                    ans += (grid[i][j] - grid[i][j - 1]).abs();
                }
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
    fn surface_area() {
        assert_eq!(Solution::surface_area(vec_vec![[1, 2], [3, 4]]), 34);
        assert_eq!(Solution::surface_area(vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]), 32);
        assert_eq!(Solution::surface_area(vec_vec![[2, 2, 2], [2, 1, 2], [2, 2, 2]]), 46);
    }
}
