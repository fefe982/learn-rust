// https://leetcode.com/problems/minimum-increment-operations-to-make-array-beautiful/
// 2919. Minimum Increment Operations to Make Array Beautiful
pub struct Solution;
impl Solution {
    pub fn min_op(nums: &[i32], start: usize, k: i32, cache: &mut Vec<i64>) -> i64 {
        if start + 3 > nums.len() {
            return 0;
        }
        if cache[start] != -1 {
            return cache[start];
        }
        let mut ik = usize::MAX;
        for i in 0..3 {
            if nums[start + i] >= k {
                ik = i;
            }
        }
        let mut ans = i64::MAX;
        if ik == usize::MAX {
            for i in 0..3 {
                ans = ans.min(Self::min_op(nums, start + i + 1, k, cache) + (k - nums[start + i]) as i64);
            }
        } else {
            ans = ans.min(Self::min_op(nums, start + ik + 1, k, cache));
        }
        cache[start] = ans;
        ans
    }
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let mut cache = vec![-1; nums.len()];
        Self::min_op(&nums, 0, k, &mut cache)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_increment_operations() {
        assert_eq!(Solution::min_increment_operations(vec![6, 2, 8, 10, 6], 9), 1);
        assert_eq!(Solution::min_increment_operations(vec![2, 3, 0, 0, 2], 4), 3);
        assert_eq!(Solution::min_increment_operations(vec![0, 1, 3, 3], 5), 2);
        assert_eq!(Solution::min_increment_operations(vec![1, 1, 2], 1), 0);
    }
}
