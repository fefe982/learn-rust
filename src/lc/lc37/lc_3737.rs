// https://leetcode.com/problems/count-subarrays-with-majority-element-i/
// 3737. Count Subarrays With Majority Element I
pub struct Solution;
impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        super::lc_3739::Solution::count_majority_subarrays(nums, target) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_majority_subarrays() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}
