// https://leetcode.com/problems/number-of-subarrays-that-match-a-pattern-ii/
// 3036. Number of Subarrays That Match a Pattern II
pub struct Solution;
impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i - 1] = match nums[i - 1].cmp(&nums[i]) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            };
        }
        nums.truncate(nums.len() - 1);
        let mut next = vec![0; pattern.len()];
        let mut i = 0;
        for j in 1..pattern.len() {
            while i > 0 && pattern[i] != pattern[j] {
                i = next[i - 1];
            }
            if pattern[i] == pattern[j] {
                i += 1;
            }
            next[j] = i;
        }
        let mut cnt = 0;
        i = 0;
        for j in 0..nums.len() {
            while i > 0 && pattern[i] != nums[j] {
                i = next[i - 1];
            }
            if pattern[i] == nums[j] {
                i += 1;
            }
            if i == pattern.len() {
                cnt += 1;
                i = next[i - 1];
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_matching_subarrays() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
