// https://leetcode.com/problems/maximum-sum-of-almost-unique-subarray/
// 2841. Maximum Sum of Almost Unique Subarray
pub struct Solution;
impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let m = m as usize;
        let k = k as usize;
        let mut cnt = std::collections::HashMap::new();
        let mut sum = 0;
        let mut max = 0;
        for i in 0..nums.len() {
            sum += nums[i] as i64;
            *cnt.entry(nums[i]).or_insert(0) += 1;
            if i >= k - 1 {
                if cnt.len() >= m {
                    max = max.max(sum);
                }
                let last = nums[i + 1 - k];
                sum -= last as i64;
                let c = cnt.get_mut(&last).unwrap();
                *c -= 1;
                if *c == 0 {
                    cnt.remove(&last);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum() {
        assert_eq!(Solution::max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
        assert_eq!(Solution::max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
        assert_eq!(Solution::max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
    }
}
