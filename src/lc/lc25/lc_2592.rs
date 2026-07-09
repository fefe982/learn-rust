// https://leetcode.com/problems/maximize-greatness-of-an-array/
// 2592. Maximize Greatness of an Array
pub struct Solution;
impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] > nums[i] {
                ans += 1;
                i += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_greatness() {
        assert_eq!(Solution::maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]), 4);
        assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4]), 3);
    }
}
