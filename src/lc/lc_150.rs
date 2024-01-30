// https://leetcode.com/problems/evaluate-reverse-polish-notation/
// 150. Evaluate Reverse Polish Notation
pub struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            if token == "+" || token == "-" || token == "*" || token == "/" {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                });
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack.pop().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_evel_rpn() {
        assert_eq!(Solution::eval_rpn(vec_str!["2", "1", "+", "3", "*"]), 9);
        assert_eq!(Solution::eval_rpn(vec_str!["4", "13", "5", "/", "+"]), 6);
        assert_eq!(
            Solution::eval_rpn(vec_str![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]),
            22
        );
    }
}
