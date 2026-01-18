// https://leetcode.com/problems/largest-magic-square/
// 1895. Largest Magic Square
pub struct Solution;
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut row_sum = vec![vec![0; n + 1]; m];
        let mut col_sum = vec![vec![0; n]; m + 1];
        for i in 0..m {
            for j in 0..n {
                row_sum[i][j + 1] = row_sum[i][j] + grid[i][j];
                col_sum[i + 1][j] = col_sum[i][j] + grid[i][j];
            }
        }
        for l in (2..=m.min(n)).rev() {
            for i in 0..=m - l {
                'j: for j in 0..=n - l {
                    let sum = row_sum[i][j + l] - row_sum[i][j];
                    for k in 1..l {
                        if sum != row_sum[i + k][j + l] - row_sum[i + k][j] {
                            continue 'j;
                        }
                    }
                    let mut sumd0 = 0;
                    let mut sumd2 = 0;
                    for k in 0..l {
                        if sum != col_sum[i + l][j + k] - col_sum[i][j + k] {
                            continue 'j;
                        }
                        sumd0 += grid[i + k][j + k];
                        sumd2 += grid[i + l - k - 1][j + k];
                    }
                    if sumd0 != sum || sumd2 != sum {
                        continue;
                    }
                    return l as i32;
                }
            }
        }
        1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn largest_magic_square() {
        assert_eq!(
            Solution::largest_magic_square(vec_vec![
                [7, 1, 4, 5, 6],
                [2, 5, 1, 6, 4],
                [1, 5, 4, 3, 2],
                [1, 2, 7, 3, 4]
            ]),
            3
        );
        assert_eq!(
            Solution::largest_magic_square(vec_vec![[5, 1, 3, 1], [9, 3, 3, 1], [1, 3, 3, 8]]),
            2
        )
    }
}
