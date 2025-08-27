// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// 1190. Reverse Substrings Between Each Pair of Parentheses
pub struct Solution;
impl Solution {
    fn process(s: &[u8], rev: bool) -> (String, usize) {
        if s.len() == 0 {
            return ("".to_string(), 0);
        }
        let mut str = String::new();
        let mut sz = 0;
        while sz < s.len() {
            let c = s[sz];
            if c == b'(' {
                let (s, szsub) = Self::process(&s[sz + 1..], !rev);
                if rev {
                    str.insert_str(0, &s);
                } else {
                    str.push_str(&s);
                }
                sz += szsub + 2;
            } else if c == b')' {
                return (str, sz);
            } else {
                if rev {
                    str.insert(0, c as char);
                } else {
                    str.push(c as char);
                }
                sz += 1;
            }
        }
        (str, sz)
    }
    pub fn reverse_parentheses(s: String) -> String {
        Self::process(s.as_bytes(), false).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_parentheses() {
        assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
        assert_eq!(Solution::reverse_parentheses("(u(love)i)".to_string()), "iloveu");
        assert_eq!(Solution::reverse_parentheses("(ed(et(oc))el)".to_string()), "leetcode");
    }
}
