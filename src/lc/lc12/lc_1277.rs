// https://leetcode.com/problems/count-square-submatrices-with-all-ones/
// 1277. Count Square Submatrices with All Ones
pub struct Solution;
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut matrix = matrix;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut one = false;
                if matrix[i][j] == 1 {
                    res += 1;
                    one = true;
                }
                if i > 0 {
                    matrix[i][j] += matrix[i - 1][j];
                }
                if j > 0 {
                    matrix[i][j] += matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    matrix[i][j] -= matrix[i - 1][j - 1];
                }
                if one {
                    for k in 2..=i.min(j) + 1 {
                        let mut n = matrix[i][j];
                        if k <= i {
                            n -= matrix[i - k][j];
                        }
                        if k <= j {
                            n -= matrix[i][j - k];
                        }
                        if k <= i && k <= j {
                            n += matrix[i - k][j - k];
                        }
                        if n == (k * k) as i32 {
                            res += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_squares() {
        assert_eq!(
            Solution::count_squares(vec_vec![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]),
            15
        );
        assert_eq!(Solution::count_squares(vec_vec![[1, 0, 1], [1, 1, 0], [1, 1, 0]]), 7);
    }
}
