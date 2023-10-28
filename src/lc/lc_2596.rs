// https://leetcode.com/problems/check-knight-tour-configuration/description/
// 2596. Check Knight Tour Configuration
pub struct Solution;
impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let mut pos = vec![(0, 0); n * n];
        if grid[0][0] != 0 {
            return false;
        }
        for i in 0..n {
            for j in 0..n {
                pos[grid[i][j] as usize] = (i as i32, j as i32);
            }
        }
        for i in 1..pos.len() {
            if ((pos[i].0 - pos[i - 1].0) * (pos[i].1 - pos[i - 1].1)).abs() != 2 {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_valid_grid() {
        assert_eq!(
            Solution::check_valid_grid(vec_vec![
                [24, 11, 22, 17, 4],
                [21, 16, 5, 12, 9],
                [6, 23, 10, 3, 18],
                [15, 20, 1, 8, 13],
                [0, 7, 14, 19, 2]
            ]),
            false
        );
        assert_eq!(
            Solution::check_valid_grid(vec_vec![
                [0, 11, 16, 5, 20],
                [17, 4, 19, 10, 15],
                [12, 1, 8, 21, 6],
                [3, 18, 23, 14, 9],
                [24, 13, 2, 7, 22]
            ]),
            true
        );
        assert_eq!(
            Solution::check_valid_grid(vec_vec![[0, 3, 6], [5, 8, 1], [2, 7, 4]]),
            false
        );
    }
}
