// https://leetcode.com/problems/maximum-width-ramp/
// 962. Maximum Width Ramp
pub struct Solution;
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::<(i32, i32)>::new();
        let mut ans = 0;
        for (n, i) in nums.into_iter().zip(0..) {
            let pos = stack.partition_point(|&(x, _)| x > n);
            if pos < stack.len() {
                ans = ans.max(i - stack[pos].1);
            } else {
                stack.push((n, i));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_width_ramp() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
    }
}
