// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
// 689. Maximum Sum of 3 Non-Overlapping Subarrays
pub struct Solution;
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut s = vec![0; n - k + 1];
        let mut left = vec![0; n - k + 1];
        for i in 0..k {
            s[0] += nums[i];
        }
        left[0] = 0;
        let mut max = s[0];
        for i in k..n {
            s[i - k + 1] = s[i - k] + nums[i] - nums[i - k];
            if s[i - k + 1] > max {
                max = s[i - k + 1];
                left[i - k + 1] = i - k + 1;
            } else {
                left[i - k + 1] = left[i - k];
            }
        }
        max = s[n - k];
        let mut right = vec![0; n - k + 1];
        right[n - k] = n - k;
        for i in (1..n - k + 1).rev() {
            if s[i - 1] >= max {
                max = s[i - 1];
                right[i - 1] = i - 1;
            } else {
                right[i - 1] = right[i];
            }
        }
        let mut max = i32::MIN;
        let mut ans = vec![0; 3];
        for i in k..n - 2 * k + 1 {
            let sum = s[i] + s[left[i - k]] + s[right[i + k]];
            if sum > max || (sum == max && left[i - k] < ans[0] as usize) {
                max = sum;
                ans = vec![left[i - k] as i32, i as i32, right[i + k] as i32];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum_of_three_subarrays() {
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
            vec![0, 3, 5]
        );
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            vec![0, 2, 4]
        );
    }
}
