// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
// 1614. Maximum Nesting Depth of the Parentheses
pub struct Solution;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max = 0;
        let mut cnt = 0;
        for c in s.chars() {
            if c == '(' {
                cnt += 1;
                max = max.max(cnt);
            } else if c == ')' {
                cnt -= 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth(String::from("(1+(2*3)+((8)/4))+1")), 3);
        assert_eq!(Solution::max_depth(String::from("(1)+((2))+(((3))))")), 3);
    }
}
