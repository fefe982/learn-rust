// https://leetcode.com/problems/maximum-xor-score-subarray-queries/
// 3277. Maximum XOR Score Subarray Queries
pub struct Solution;
impl Solution {
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut x = vec![vec![0; n]; n];
        for i in 0..n {
            x[i][i] = nums[i];
            for j in (0..i).rev() {
                x[j][i] = x[j][i - 1] ^ x[j + 1][i];
            }
        }
        for i in 0..n {
            for j in i + 1..n {
                x[i][j] = x[i][j].max(x[i][j - 1]);
            }
        }
        for i in 0..n {
            for j in (0..i).rev() {
                x[j][i] = x[j][i].max(x[j + 1][i]);
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            ans.push(x[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_subarray_xor() {
        assert_eq!(
            Solution::maximum_subarray_xor(vec![2, 8, 4, 32, 16, 1], vec_vec![[0, 2], [1, 4], [0, 5]]),
            [12, 60, 60]
        );
        assert_eq!(
            Solution::maximum_subarray_xor(
                vec![0, 7, 3, 2, 8, 5, 1],
                vec_vec![[0, 3], [1, 5], [2, 4], [2, 6], [5, 6]]
            ),
            [7, 14, 11, 14, 5]
        );
    }
}
