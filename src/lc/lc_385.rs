// https://leetcode.com/problems/mini-parser/
// 385. Mini Parser
pub struct Solution;
use super::nested_integer::NestedInteger;
impl Solution {
    fn parse_number(s: &[u8]) -> (NestedInteger, &[u8]) {
        let mut i = 0;
        let mut sign = 1;
        let mut n = 0;
        if s[0] == b'-' {
            sign = -1;
            i += 1;
        }
        while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
            n = n * 10 + (s[i] - b'0') as i32;
            i += 1;
        }
        (NestedInteger::Int(sign * n), &s[i..])
    }
    fn parse_list(mut s: &[u8]) -> (NestedInteger, &[u8]) {
        let mut v = vec![];
        while s[0] != b']' {
            match s[0] {
                b',' => s = &s[1..],
                _ => {
                    let (ele, ns) = Self::parse_ele(s);
                    v.push(ele);
                    s = ns
                }
            }
        }
        (NestedInteger::List(v), &s[1..])
    }
    fn parse_ele(s: &[u8]) -> (NestedInteger, &[u8]) {
        if s[0] == b'[' {
            Self::parse_list(&s[1..])
        } else {
            Self::parse_number(s)
        }
    }
    pub fn deserialize(s: String) -> NestedInteger {
        let s = s.as_bytes();
        Self::parse_ele(s).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize() {
        // assert_eq!(Solution::deserialize("324".to_string()), NestedInteger::Int(324));
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_string()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ])
        );
    }
}
