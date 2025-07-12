// https://leetcode.com/problems/score-of-parentheses/
// 856. Score of Parentheses
pub struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut v = vec![0];
        for c in s.chars() {
            match c {
                '(' => v.push(0),
                ')' => {
                    let last = v.pop().unwrap();
                    if last == 0 {
                        *v.last_mut().unwrap() += 1;
                    } else {
                        *v.last_mut().unwrap() += 2 * last;
                    }
                }
                _ => {}
            }
        }
        v[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_of_parentheses() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
    }
}
