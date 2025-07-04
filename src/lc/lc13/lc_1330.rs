// https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/
// 1330. Reverse Subarray To Maximize Array Value
pub struct Solution;
impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut bound = 0;
        let mut mx = i32::MIN;
        let mut mn = i32::MAX;
        for i in 1..nums.len() {
            let d = (nums[i - 1] - nums[i]).abs();
            sum += d;
            mx = std::cmp::max(mx, std::cmp::min(nums[i], nums[i - 1]));
            mn = std::cmp::min(mn, std::cmp::max(nums[i], nums[i - 1]));
            bound = std::cmp::max(
                bound,
                std::cmp::max(
                    (nums[i - 1] - nums[nums.len() - 1]).abs() - d,
                    (nums[0] - nums[i]).abs() - d,
                ),
            );
        }
        sum + std::cmp::max(bound, 2 * (mx - mn))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_value_after_reverse() {
        assert_eq!(Solution::max_value_after_reverse(vec![-1, -2]), 1);
        assert_eq!(Solution::max_value_after_reverse(vec![2, 3, 1, 5, 4]), 10);
        assert_eq!(Solution::max_value_after_reverse(vec![2, 4, 9, 24, 2, 1, 10]), 68);
    }
}
