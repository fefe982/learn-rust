// https://leetcode.com/problems/maximum-erasure-value/
// 1695. Maximum Erasure Value
pub struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut left = 0;
        let mut max = 0;
        let mut sum = 0;
        for &v in nums.iter() {
            while set.contains(&v) {
                set.remove(&nums[left]);
                sum -= nums[left];
                left += 1;
            }
            set.insert(v);
            sum += v;
            max = max.max(sum);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_unique_subarray() {
        assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
        assert_eq!(Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
    }
}
