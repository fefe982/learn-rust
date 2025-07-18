// https://leetcode.com/problems/projection-area-of-3d-shapes/
// 883. Projection Area of 3D Shapes
pub struct Solution;
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut m = vec![0; n];
        let mut s = 0;
        for i in 0..n {
            let mut max = 0;
            for j in 0..n {
                if grid[i][j] > 0 {
                    s += 1;
                }
                max = max.max(grid[i][j]);
                m[j] = m[j].max(grid[i][j]);
            }
            s += max;
        }
        s + m.iter().sum::<i32>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn projection_area() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
