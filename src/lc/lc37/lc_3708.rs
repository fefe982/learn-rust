// https://leetcode.com/problems/longest-fibonacci-subarray/
// 3708. Longest Fibonacci Subarray
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }
        let mut ans = 2;
        let mut c = 2;
        for i in 2..nums.len() {
            if nums[i] == nums[i - 1] + nums[i - 2] {
                c += 1;
                ans = ans.max(c);
            } else {
                c = 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]), 5);
        assert_eq!(Solution::longest_subarray(vec![5, 2, 7, 9, 16]), 5);
        assert_eq!(Solution::longest_subarray(vec![1000000000, 1000000000, 1000000000]), 2);
    }
}
