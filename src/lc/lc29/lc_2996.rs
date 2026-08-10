// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum/
// 2996. Smallest Missing Non-negative Integer After Operations
pub struct Solution;
impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut s = nums[0];
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                s += nums[i];
            } else {
                break;
            }
        }
        nums.sort_unstable();
        let mut i = nums.partition_point(|&x| x < s);
        while i < nums.len() && nums[i] == s {
            while i < nums.len() && nums[i] == s {
                i += 1;
            }
            s += 1;
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_integer() {
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }
}
