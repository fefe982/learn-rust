// https://leetcode.com/problems/maximum-trailing-zeros-in-a-cornered-path/
// 2245. Maximum Trailing Zeros in a Cornered Path
pub struct Solution;
impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut count_2_col = vec![vec![0; n + 1]; m + 1];
        let mut count_2_row = vec![vec![0; n + 1]; m + 1];
        let mut count_5_col = vec![vec![0; n + 1]; m + 1];
        let mut count_5_row = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                let n2 = grid[i][j].trailing_zeros() as i32;
                count_2_col[i + 1][j + 1] = count_2_col[i + 1][j] + n2;
                count_2_row[i + 1][j + 1] = count_2_row[i][j + 1] + n2;
                let mut n5 = 0;
                let mut x = grid[i][j];
                while x % 5 == 0 {
                    n5 += 1;
                    x /= 5;
                }
                count_5_col[i + 1][j + 1] = count_5_col[i + 1][j] + n5;
                count_5_row[i + 1][j + 1] = count_5_row[i][j + 1] + n5;
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let l2 = count_2_row[i + 1][j + 1];
                let r2 = count_2_row[m][j + 1] - count_2_row[i][j + 1];
                let u2 = count_2_col[i + 1][j];
                let d2 = count_2_col[i + 1][n] - count_2_col[i + 1][j + 1];
                let l5 = count_5_row[i + 1][j + 1];
                let r5 = count_5_row[m][j + 1] - count_5_row[i][j + 1];
                let u5 = count_5_col[i + 1][j];
                let d5 = count_5_col[i + 1][n] - count_5_col[i + 1][j + 1];
                ans = ans
                    .max((l2 + u2).min(l5 + u5))
                    .max((l2 + d2).min(l5 + d5))
                    .max((r2 + u2).min(r5 + u5))
                    .max((r2 + d2).min(r5 + d5));
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
    fn max_trailing_zeros() {
        assert_eq!(
            Solution::max_trailing_zeros(vec_vec![
                [23, 17, 15, 3, 20],
                [8, 1, 20, 27, 11],
                [9, 4, 6, 2, 21],
                [40, 9, 1, 10, 6],
                [22, 7, 4, 5, 3]
            ]),
            3
        );
        assert_eq!(
            Solution::max_trailing_zeros(vec_vec![[4, 3, 2], [7, 6, 1], [8, 8, 8]]),
            0
        );
    }
}
