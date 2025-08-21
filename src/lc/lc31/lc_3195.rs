// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/
// 3195. Find the Minimum Area to Cover All Ones
pub struct Solution;
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut mini = usize::MAX;
        let mut maxi = 0;
        let mut minj = usize::MAX;
        let mut maxj = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    mini = mini.min(i);
                    maxi = maxi.max(i);
                    minj = minj.min(j);
                    maxj = maxj.max(j);
                }
            }
        }
        (maxi - mini + 1) as i32 * (maxj - minj + 1) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_area() {
        assert_eq!(Solution::minimum_area(vec_vec![[0, 1, 0], [1, 0, 1]]), 6);
        assert_eq!(Solution::minimum_area(vec_vec![[1, 0], [0, 0]]), 1);
    }
}
