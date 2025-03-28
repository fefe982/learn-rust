// https://leetcode.com/problems/rotate-function/
// 396. Rotate Function
pub struct Solution;
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut s = 0;
        for i in 0..nums.len() {
            m += i as i32 * nums[i];
            s += nums[i];
        }
        let mut max = m;
        for i in (1..nums.len()).rev() {
            m = m - nums[i] * nums.len() as i32 + s;
            max = max.max(m);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_rotate_function() {
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
        assert_eq!(Solution::max_rotate_function(vec![100]), 0);
    }
}
