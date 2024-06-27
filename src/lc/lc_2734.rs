// https://leetcode.com/problems/lexicographically-smallest-string-after-substring-operation/
// 2734. Lexicographically Smallest String After Substring Operation
pub struct Solution;
impl Solution {
    pub fn smallest_string(s: String) -> String {
        #[derive(PartialEq, Eq)]
        enum State {
            Begin,
            Processing,
            End,
        }
        let mut res = "".to_string();
        let mut state = State::Begin;
        for c in s.bytes() {
            match state {
                State::Begin => {
                    if c == b'a' {
                        res.push('a');
                    } else {
                        res.push((c - 1) as char);
                        state = State::Processing;
                    }
                }
                State::Processing => {
                    if c == b'a' {
                        res.push('a');
                        state = State::End;
                    } else {
                        res.push((c - 1) as char);
                    }
                }
                State::End => res.push(c as char),
            }
        }
        if state == State::Begin {
            res.pop();
            res.push('z');
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_string() {
        assert_eq!(Solution::smallest_string("cbabc".to_string()), "baabc");
        assert_eq!(Solution::smallest_string("aa".to_string()), "az");
        assert_eq!(Solution::smallest_string("acbbc".to_string()), "abaab");
        assert_eq!(Solution::smallest_string("leetcode".to_string()), "kddsbncd");
    }
}
