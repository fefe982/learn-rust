// https://leetcode.com/problems/sum-of-good-numbers/
// 3452. Sum of Good Numbers
pub struct Solution;
impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let k = k as usize;
        for i in 0..nums.len() {
            if (i < k || nums[i] > nums[i - k]) && (i + k >= nums.len() || nums[i] > nums[i + k]) {
                sum += nums[i];
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_good_numbers() {
        assert_eq!(Solution::sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2), 12);
        assert_eq!(Solution::sum_of_good_numbers(vec![2, 1], 1), 2);
    }
}
