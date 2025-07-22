// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
// 921. Minimum Add to Make Parentheses Valid
pub struct Solution;
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut cnt = 0;
        let mut min = 0;
        for c in s.chars() {
            if c == '(' {
                cnt += 1;
            } else {
                cnt -= 1;
                min = min.min(cnt);
            }
        }
        cnt - 2 * min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_add_to_make_valid() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    }
}
