// https://leetcode.com/problems/matrix-block-sum/
// 1314. Matrix Block Sum
pub struct Solution;
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                sum[i][j] = mat[i - 1][j - 1] + sum[i - 1][j] + sum[i][j - 1] - sum[i - 1][j - 1];
            }
        }
        let get = |r: i32, c: i32| {
            let mut sr = 0;
            let mut sc = 0;
            if r > 0 && r <= m as i32 {
                sr = r as usize;
            } else if r > m as i32 {
                sr = m;
            }
            if c > 0 && c <= n as i32 {
                sc = c as usize;
            } else if c > n as i32 {
                sc = n;
            }
            sum[sr][sc]
        };
        let mut res = vec![vec![0; n]; m];
        for i in 0..m as i32 {
            for j in 0..n as i32 {
                res[i as usize][j as usize] =
                    get(i + k + 1, j + k + 1) - get(i - k, j + k + 1) - get(i + k + 1, j - k) + get(i - k, j - k);
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
    fn matrix_block_sum() {
        assert_eq!(
            Solution::matrix_block_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 1),
            vec_vec![[12, 21, 16], [27, 45, 33], [24, 39, 28]]
        );
        assert_eq!(
            Solution::matrix_block_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 2),
            vec_vec![[45, 45, 45], [45, 45, 45], [45, 45, 45]]
        );
    }
}
