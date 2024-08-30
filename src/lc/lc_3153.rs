// https://leetcode.com/problems/sum-of-digit-differences-of-all-pairs/
// 3153. Sum of Digit Differences of All Pairs
pub struct Solution;
impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut cnt = vec![0; 10];
        let mut nums = nums;
        let len = nums.len() as i64;
        while nums[0] > 0 {
            cnt.fill(0);
            for n in nums.iter_mut() {
                let d = *n % 10;
                *n /= 10;
                cnt[d as usize] += 1;
            }
            for i in 0..10 {
                ans += cnt[i] as i64 * (len - cnt[i] as i64)
            }
        }
        ans / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_digit_differences() {
        assert_eq!(
            Solution::sum_digit_differences(vec![824, 843, 837, 620, 836, 234, 276, 859]),
            67
        );
        assert_eq!(Solution::sum_digit_differences(vec![13, 23, 12]), 4);
        assert_eq!(Solution::sum_digit_differences(vec![10, 10, 10, 10]), 0);
    }
}
