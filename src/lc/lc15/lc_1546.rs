// https://leetcode.com/problems/maximum-number-of-non-overlapping-subarrays-with-sum-equals-target/
// 1546. Maximum Number of Non-Overlapping Subarrays With Sum Equals Target
pub struct Solution;
impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt = Vec::with_capacity(nums.len() + 1);
        let mut map = std::collections::HashMap::new();
        let mut c = 0;
        let mut sum = 0;
        cnt.push(0);
        for i in 0..nums.len() {
            map.insert(sum, i);
            sum += nums[i];
            if let Some(&j) = map.get(&(sum - target)) {
                c = c.max(cnt[j] + 1);
            }
            cnt.push(c);
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_non_overlapping() {
        assert_eq!(Solution::max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2);
        assert_eq!(Solution::max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6), 2);
    }
}
