// https://leetcode.com/problems/sum-of-variable-length-subarrays/
// 3427. Sum of Variable Length Subarrays
pub struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut nums = nums;
        let mut ans = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            let ns = nums[i] as usize;
            nums[i] = sum;
            if i > ns {
                ans += sum - nums[i - ns - 1];
            } else {
                ans += sum;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subarray_sum() {
        assert_eq!(Solution::subarray_sum(vec![2, 3, 1]), 11);
        assert_eq!(Solution::subarray_sum(vec![3, 1, 1, 2]), 13);
    }
}
