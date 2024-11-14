// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i/
// 3239. Minimum Number of Flips to Make Binary Grid Palindromic
pub struct Solution;
impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut flip1 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() / 2 {
                if grid[i][j] != grid[i][grid[0].len() - j - 1] {
                    flip1 += 1;
                }
            }
        }
        let mut flip2 = 0;
        for i in 0..grid.len() / 2 {
            for j in 0..grid[0].len() {
                if grid[i][j] != grid[grid.len() - i - 1][j] {
                    flip2 += 1;
                }
            }
        }
        flip1.min(flip2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_flips() {
        assert_eq!(Solution::min_flips(vec_vec![[1, 0, 0], [0, 0, 0], [0, 0, 1]]), 2);
        assert_eq!(Solution::min_flips(vec_vec![[0, 1], [0, 1], [0, 0]]), 1);
        assert_eq!(Solution::min_flips(vec_vec![[1], [0]]), 0);
    }
}
