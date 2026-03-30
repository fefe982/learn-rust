// https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/
// 1807. Evaluate the Bracket Pairs of a String
pub struct Solution;
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut dict = HashMap::with_capacity(knowledge.len());
        for pair in knowledge {
            let mut it = pair.into_iter();
            if let (Some(k), Some(v)) = (it.next(), it.next()) {
                dict.insert(k, v);
            }
        }

        let bytes = s.as_bytes();
        let mut ans = String::new();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'(' {
                let mut j = i + 1;
                while bytes[j] != b')' {
                    j += 1;
                }

                let key = &s[i + 1..j];
                if let Some(v) = dict.get(key) {
                    ans.push_str(v);
                } else {
                    ans.push('?');
                }
                i = j + 1;
            } else {
                ans.push(bytes[i] as char);
                i += 1;
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn evaluate() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)yearsold".to_string(),
                vec_vec_str![["name", "bob"], ["age", "two"]]
            ),
            "bobistwoyearsold".to_string()
        );
        assert_eq!(
            Solution::evaluate("hi(name)".to_string(), vec_vec_str![["a", "b"]]),
            "hi?".to_string()
        );
        assert_eq!(
            Solution::evaluate("(a)(a)(a)aaa".to_string(), vec_vec_str![["a", "yes"]]),
            "yesyesyesaaa".to_string()
        );
    }
}
