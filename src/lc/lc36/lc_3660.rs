// https://leetcode.com/problems/jump-game-ix/
// 3660. Jump Game IX
pub struct Solution;
impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }

        let mut suffix_min = vec![0; n];
        suffix_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i]);
        }

        let mut ans = vec![0; n];
        let mut start = 0usize;
        let mut comp_max = nums[0];

        for i in 0..n - 1 {
            comp_max = comp_max.max(nums[i]);
            if comp_max <= suffix_min[i + 1] {
                for v in ans.iter_mut().take(i + 1).skip(start) {
                    *v = comp_max;
                }
                start = i + 1;
                comp_max = nums[start];
            }
        }

        comp_max = comp_max.max(nums[n - 1]);
        for v in ans.iter_mut().take(n).skip(start) {
            *v = comp_max;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_value() {
        assert_eq!(Solution::max_value(vec![2, 1, 3]), vec![2, 2, 3]);
        assert_eq!(Solution::max_value(vec![2, 3, 1]), vec![3, 3, 3]);
        assert_eq!(Solution::max_value(vec![1, 2, 2, 1]), vec![1, 2, 2, 2]);
        assert_eq!(Solution::max_value(vec![2, 1, 4, 3]), vec![2, 2, 4, 4]);
    }
}
