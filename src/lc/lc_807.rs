// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
// 807. Max Increase to Keep City Skyline
pub struct Solution;
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut rmax = vec![0; n];
        let mut cmax = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                rmax[i] = rmax[i].max(grid[i][j]);
                cmax[j] = cmax[j].max(grid[i][j]);
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += rmax[i].min(cmax[j]) - grid[i][j];
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
    fn test_max_increase_keeping_skyline() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec_vec![[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]]),
            35
        );
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec_vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]]),
            0
        )
    }
}
