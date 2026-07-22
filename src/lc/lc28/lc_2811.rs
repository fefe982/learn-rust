// https://leetcode.com/problems/check-if-it-is-possible-to-split-array/
// 2811. Check If It Is Possible to Split String Into Palindromes II
pub struct Solution;
impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        let n = nums.len();
        if n <= 2 {
            return true;
        }
        for i in 1..n {
            if nums[i] + nums[i - 1] >= m {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_split_array() {
        assert_eq!(Solution::can_split_array(vec![2, 2, 1], 4), true);
        assert_eq!(Solution::can_split_array(vec![2, 1, 3], 5), false);
        assert_eq!(Solution::can_split_array(vec![2, 3, 3, 2, 3], 6), true);
    }
}
