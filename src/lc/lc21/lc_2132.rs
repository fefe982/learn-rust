// https://leetcode.com/problems/stamping-the-grid/
// 2132. Stamping the Grid
pub struct Solution;
impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let h = grid.len();
        let w = grid[0].len();
        let sh = stamp_height as usize;
        let sw = stamp_width as usize;
        let mut sum = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + grid[i][j];
            }
        }
        let mut diff = vec![vec![0; w + 2]; h + 2];
        for i in 0..h {
            for j in 0..w {
                if i + sh <= h && j + sw <= w && sum[i + sh][j + sw] - sum[i][j + sw] - sum[i + sh][j] + sum[i][j] == 0
                {
                    diff[i + 1][j + 1] += 1;
                    diff[i + sh + 1][j + 1] -= 1;
                    diff[i + 1][j + sw + 1] -= 1;
                    diff[i + sh + 1][j + sw + 1] += 1;
                }
                diff[i + 1][j + 1] = diff[i][j + 1] + diff[i + 1][j] - diff[i][j] + diff[i + 1][j + 1];
                if grid[i][j] == 0 && diff[i + 1][j + 1] == 0 {
                    return false;
                }
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
    fn test_possible_to_stamp() {
        assert_eq!(
            Solution::possible_to_stamp(
                vec_vec![[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
                4,
                3
            ),
            true
        );
        assert_eq!(
            Solution::possible_to_stamp(vec_vec![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]], 2, 2),
            false
        );
    }
}
