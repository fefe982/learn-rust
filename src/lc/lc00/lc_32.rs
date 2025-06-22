// https://leetcode.com/problems/longest-valid-parentheses/
// 32. Longest Valid Parentheses
pub struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut current_len = 0;
        let mut max_len = 0;
        let mut open_stack = Vec::new();
        for (idx, &c) in s.as_bytes().iter().enumerate() {
            if c == b'(' {
                open_stack.push(idx);
            } else {
                if let Some(last_open) = open_stack.pop() {
                    let mut this_len = idx - last_open + 1;
                    if open_stack.len() == 0 {
                        current_len += this_len;
                        this_len = current_len;
                    } else {
                        this_len = idx - open_stack[open_stack.len() - 1];
                    }
                    if this_len > max_len {
                        max_len = this_len;
                    }
                } else {
                    current_len = 0;
                }
            }
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_valid_parentheses() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("(()()")),
            4
        );
        assert_eq!(Solution::longest_valid_parentheses(String::from("(()")), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")()())")),
            4
        );
        assert_eq!(Solution::longest_valid_parentheses(String::from("")), 0);
    }
}
