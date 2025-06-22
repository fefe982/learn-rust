// https://leetcode.com/problems/k-radius-subarray-averages/
// 2090. K Radius Subarray Averages
pub struct Solution;
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if nums.len() < 2 * k + 1 {
            return vec![-1; nums.len()];
        }
        let mut res = Vec::with_capacity(nums.len());
        let mut sum: i64 = 0;
        for i in 0..k {
            sum += (nums[i] + nums[i + k]) as i64;
            res.push(-1);
        }
        for i in k..nums.len() - k {
            sum += nums[i + k] as i64;
            res.push((sum / (2 * k as i64 + 1)) as i32);
            sum -= nums[i - k] as i64;
        }
        for _ in nums.len() - k..nums.len() {
            res.push(-1);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_averages() {
        assert_eq!(
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
            [-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
        assert_eq!(Solution::get_averages(vec![10000], 0), [10000]);
        assert_eq!(Solution::get_averages(vec![8], 10), [-1]);
    }
}
