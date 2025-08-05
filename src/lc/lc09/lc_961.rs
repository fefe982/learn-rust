// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
// 961. N-Repeated Element in Size 2N Array
pub struct Solution;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len() / 2;
        if nums[n] == nums[n + 1] {
            nums[n]
        } else {
            nums[n - 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_n_times() {
        assert_eq!(Solution::repeated_n_times(vec![2, 6, 2, 1]), 2);
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
