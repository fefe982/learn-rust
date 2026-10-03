// https://leetcode.com/problems/zero-array-transformation-iv/
// 3489. Zero Array Transformation IV
pub struct Solution;
impl Solution {
    fn solve(q: &Vec<Vec<i32>>, i: usize, t: i32, j: usize, dp: &mut Vec<Vec<usize>>) -> usize {
        if t == 0 {
            return j;
        }
        if j >= q.len() || t < 0 {
            return q.len() + 1;
        }
        if dp[t as usize][j] != usize::MAX {
            return dp[t as usize][j];
        }
        let mut r = Self::solve(q, i, t, j + 1, dp);
        if q[j][0] as usize <= i && i <= q[j][1] as usize {
            r = r.min(Self::solve(q, i, t - q[j][2], j + 1, dp));
        }
        dp[t as usize][j] = r;
        r
    }
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res = res.max(Self::solve(
                &queries,
                i,
                nums[i],
                0,
                &mut vec![vec![usize::MAX; queries.len()]; (nums[i] + 1) as usize],
            ));
        }
        if res > queries.len() {
            -1
        } else {
            res as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_zero_array() {
        assert_eq!(
            Solution::min_zero_array(vec![2, 0, 2], vec_vec![[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
            2
        );
        assert_eq!(
            Solution::min_zero_array(vec![4, 3, 2, 1], vec_vec![[1, 3, 2], [0, 2, 1]]),
            -1
        );
        assert_eq!(
            Solution::min_zero_array(
                vec![1, 2, 3, 2, 1],
                vec_vec![[0, 1, 1], [1, 2, 1], [2, 3, 2], [3, 4, 1], [4, 4, 1]]
            ),
            4
        );
        assert_eq!(
            Solution::min_zero_array(
                vec![1, 2, 3, 2, 6],
                vec_vec![[0, 1, 1], [0, 2, 1], [1, 4, 2], [4, 4, 4], [3, 4, 1], [4, 4, 5]]
            ),
            4
        );
    }
}
