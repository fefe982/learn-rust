// https://leetcode.com/problems/remove-outermost-parentheses/
// 1021. Remove Outermost Parentheses
pub struct Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::new();
        let mut i = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    if i > 0 {
                        res.push(c);
                    }
                    i += 1;
                }
                ')' => {
                    i -= 1;
                    if i > 0 {
                        res.push(c);
                    }
                }
                _ => {}
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_outer_parentheses() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "".to_string());
    }
}
