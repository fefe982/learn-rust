// https://leetcode.com/problems/lexicographically-smallest-string-after-reverse/
// 3722. Lexicographically Smallest String After Reverse
pub struct Solution;
impl Solution {
    pub fn lex_smallest(s: String) -> String {
        let mut min = s.clone();
        for i in 2..=s.len() {
            let sl = s[..i].chars().rev().chain(s[i..].chars()).collect::<String>();
            if sl < min {
                min = sl;
            }
            let sr = s[..s.len() - i]
                .chars()
                .chain(s[s.len() - i..].chars().rev())
                .collect::<String>();
            if sr < min {
                min = sr;
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_smallest() {
        assert_eq!(Solution::lex_smallest("dcab".to_string()), "acdb".to_string());
        assert_eq!(Solution::lex_smallest("abba".to_string()), "aabb".to_string());
        assert_eq!(Solution::lex_smallest("zxy".to_string()), "xzy".to_string());
    }
}
