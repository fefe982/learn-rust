// https://leetcode.com/problems/maximum-average-subarray-i/
// 643. Maximum Average Subarray I
pub struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = 0;
        let k = k as usize;
        for i in 0..k {
            max += nums[i];
        }
        let mut val = max;
        for j in k..nums.len() {
            val += nums[j] - nums[j - k];
            max = max.max(val);
        }
        max as f64 / k as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;
    #[test]
    fn find_max_average() {
        assert_approx_eq!(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
        assert_approx_eq!(Solution::find_max_average(vec![5], 1), 5.0);
    }
}
