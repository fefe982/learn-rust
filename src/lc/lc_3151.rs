// https://leetcode.com/problems/special-array-i/
// 3151. Special Array I
pub struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let m = nums[0] % 2;
        for i in 1..nums.len() {
            if (nums[i] + i as i32) % 2 != m {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_array_special() {
        assert_eq!(Solution::is_array_special(vec![1]), true);
        assert_eq!(Solution::is_array_special(vec![2, 1, 4]), true);
        assert_eq!(Solution::is_array_special(vec![4, 3, 1, 6]), false);
    }
}
