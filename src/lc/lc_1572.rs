// https://leetcode.com/problems/matrix-diagonal-sum/
// 1572. Matrix Diagonal Sum
pub struct Solution;
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let l = mat.len();

        for idx in 0..(l + 1) / 2 {
            sum += mat[idx][idx];
            let midx = l - idx - 1;
            if midx != idx {
                sum += mat[idx][midx];
                sum += mat[midx][idx];
                sum += mat[midx][midx];
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn diagonal_sum() {
        assert_eq!(
            Solution::diagonal_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            25
        );
        assert_eq!(
            Solution::diagonal_sum(vec_vec![
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1]
            ]),
            8
        );
        assert_eq!(Solution::diagonal_sum(vec_vec![[5]]), 5);
    }
}
