// https://leetcode.com/problems/minimum-average-of-smallest-and-largest-elements/
// 3194. Minimum Average of Subarrays With Size K
pub struct Solution;
impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = f64::MAX;
        for i in 0..nums.len() / 2 {
            res = res.min((nums[i] + nums[nums.len() - 1 - i]) as f64 / 2.0);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_average() {
        assert_eq!(Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]), 5.5);
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
