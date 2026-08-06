// https://leetcode.com/problems/length-of-the-longest-subsequence-that-sums-to-target/
// 2915. Length of the Longest Subsequence That Sums to Target
pub struct Solution;
impl Solution {
    fn search(nums: &[i32], target: i32, cache: &mut Vec<Vec<i32>>, index: usize) -> i32 {
        if target == 0 {
            return 0;
        }
        if index >= nums.len() {
            return i32::MIN;
        }
        if cache[index][target as usize] != 0 {
            return cache[index][target as usize];
        }
        let mut max_len = Self::search(nums, target, cache, index + 1);
        if nums[index] <= target {
            max_len = max_len.max(1 + Self::search(nums, target - nums[index], cache, index + 1))
        }
        cache[index][target as usize] = max_len;
        max_len
    }
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let max_len = Self::search(&nums, target, &mut vec![vec![0; (target + 1) as usize]; nums.len()], 0);
        if max_len < 0 {
            -1
        } else {
            max_len
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_longest_subsequence() {
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9), 3);
        assert_eq!(Solution::length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7), 4);
        assert_eq!(Solution::length_of_longest_subsequence(vec![1, 1, 5, 4, 5], 3), -1);
    }
}
