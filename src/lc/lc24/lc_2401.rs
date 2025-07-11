// https://leetcode.com/problems/longest-nice-subarray/
// 2401. Longest Nice Subarray
pub struct Solution;
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut i = 0;
        let mut j = 0;
        let mut max = 1;
        while j < nums.len() {
            while j < nums.len() && nums[j] & s == 0 {
                s |= nums[j];
                j += 1;
            }
            max = max.max(j - i);
            s ^= nums[i];
            i += 1;
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_nice_subarray() {
        assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
        assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    }
}
