// https://leetcode.com/problems/generate-parentheses/
// 22. Generate Parentheses
pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut full = vec![vec![]; n as usize];
        let mut part = vec![vec![]; n as usize];
        full[0].push("".to_string());
        let n = n as usize;
        for i in 1..=n {
            let mut p = vec![];
            for j in 1..i {
                for s in &full[j] {
                    for a in full[i - j].iter().chain(&part[i - j]) {
                        p.push(format!("{}{}", s, a));
                    }
                }
            }
            if i < n {
                part[i] = p;
                p = vec![];
            }
            for a in full[i - 1].iter().chain(&part[i - 1]) {
                p.push(format!("({})", a));
            }
            if i == n {
                return p;
            } else {
                full[i] = p;
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_generate_parenthesis() {
        assert_sort_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_sort_eq!(Solution::generate_parenthesis(2), vec!["()()", "(())"]);
        assert_sort_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_sort_eq!(
            Solution::generate_parenthesis(4),
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()", "(())(())",
                "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }
}
