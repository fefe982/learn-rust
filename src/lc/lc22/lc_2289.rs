// https://leetcode.com/problems/steps-to-make-array-non-decreasing/
// 2289. Steps to Make Array Non-decreasing
pub struct Solution;
impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut ans = 0;
        for n in nums.into_iter().rev() {
            let mut step = 0;
            while let Some((prev, prev_step)) = stack.last() {
                if *prev >= n {
                    break;
                }
                step += 1;
                step = step.max(*prev_step);
                stack.pop();
            }
            ans = ans.max(step);
            stack.push((n, step));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_steps() {
        assert_eq!(Solution::total_steps(vec![10, 6, 5, 10, 15]), 1);
        assert_eq!(Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
        assert_eq!(Solution::total_steps(vec![4, 5, 7, 7, 13]), 0);
    }
}
