// https://leetcode.com/problems/remove-invalid-parentheses/description/
// 301. Remove Invalid Parentheses
pub struct Solution;
impl Solution {
    fn count(s: &Vec<u8>) -> (i32, i32) {
        let mut b = 0;
        let mut r = 0;
        for &c in s {
            match c {
                b'(' => b += 1,
                b')' => {
                    if b == 0 {
                        r += 1;
                    } else {
                        b -= 1;
                    }
                }
                _ => (),
            }
        }
        (b, r)
    }
    fn valid(s: &Vec<u8>) -> bool {
        let mut b = 0;
        for &c in s {
            match c {
                b'(' => b += 1,
                b')' => {
                    if b == 0 {
                        return false;
                    } else {
                        b -= 1;
                    }
                }
                _ => (),
            }
        }
        b == 0
    }
    fn remove(s: Vec<u8>, from: usize, left: i32, right: i32, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            if Self::valid(&s) {
                res.push(String::from_utf8(s).unwrap());
            }
            return;
        }
        for i in from..s.len() {
            if i > from && s[i] == s[i - 1] {
                continue;
            }
            if s[i] == b'(' && left > 0 {
                let ns: Vec<u8> = s[0..i].iter().chain(s[i + 1..].iter()).cloned().collect();
                Self::remove(ns, i, left - 1, right, res);
            } else if s[i] == b')' && right > 0 {
                let ns: Vec<u8> = s[0..i].iter().chain(s[i + 1..].iter()).cloned().collect();
                Self::remove(ns, i, left, right - 1, res);
            }
        }
    }
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s = s.as_bytes().to_vec();
        let (left, right) = Self::count(&s);
        let mut res = Vec::new();
        Self::remove(s, 0, left, right, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn remove_invalid_parentheses() {
        assert_eq!(
            Solution::remove_invalid_parentheses(String::from("()())()")),
            vec_str!["(())()", "()()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(String::from("(a)())()")),
            ["(a())()", "(a)()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(String::from(")(")),
            [""]
        );
    }
}
