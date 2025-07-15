// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii/
// 3240. Minimum Number of Flips to Make Binary Grid Palindromic II
pub struct Solution;
impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut flip = 0;
        for i in 0..grid.len() / 2 {
            for j in 0..grid[0].len() / 2 {
                let sum = grid[i][j]
                    + grid[i][grid[0].len() - j - 1]
                    + grid[grid.len() - i - 1][j]
                    + grid[grid.len() - i - 1][grid[0].len() - j - 1];
                if sum == 2 {
                    flip += 2;
                } else if sum % 2 == 1 {
                    flip += 1;
                }
            }
        }
        if grid.len() % 2 == 1 && grid[0].len() % 2 == 1 {
            flip += grid[grid.len() / 2][grid[0].len() / 2];
        }
        let mut ones = 0;
        let mut flip2 = 0;
        if grid.len() % 2 == 1 {
            for i in 0..grid[0].len() / 2 {
                if grid[grid.len() / 2][i] != grid[grid.len() / 2][grid[0].len() - i - 1] {
                    flip2 += 1;
                } else if grid[grid.len() / 2][i] == 1 {
                    ones += 1;
                }
            }
        }
        if grid[0].len() % 2 == 1 {
            for i in 0..grid.len() / 2 {
                if grid[i][grid[0].len() / 2] != grid[grid.len() - i - 1][grid[0].len() / 2] {
                    flip2 += 1;
                } else if grid[i][grid[0].len() / 2] == 1 {
                    ones += 1;
                }
            }
        }
        if ones % 2 == 1 && flip2 == 0 {
            flip2 = 2;
        }
        flip + flip2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_flips() {
        assert_eq!(
            Solution::min_flips(vec_vec![[1, 0, 1], [0, 0, 0], [0, 0, 0], [0, 0, 1]]),
            1
        );
        assert_eq!(
            Solution::min_flips(vec_vec![[0, 0, 0], [1, 1, 0], [0, 1, 1], [0, 0, 1]]),
            5
        );
        assert_eq!(Solution::min_flips(vec_vec![[1], [1], [1], [0]]), 1);
        assert_eq!(Solution::min_flips(vec_vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]]), 3);
        assert_eq!(Solution::min_flips(vec_vec![[0, 1], [0, 1], [0, 0]]), 2);
        assert_eq!(Solution::min_flips(vec_vec![[1], [1]]), 2);
    }
}
