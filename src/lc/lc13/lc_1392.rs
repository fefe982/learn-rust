// https://leetcode.com/problems/longest-happy-prefix/
// 1392. Longest Happy Prefix
pub struct Solution;
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut v = vec![];
        let s = s.as_bytes();
        let m = 1_0000_0000_7;
        let mut pow = 1;
        let mut l = 0;
        let mut r = 0;
        for i in 0..s.len() - 1 {
            let lc = (s[i] - b'a') as i64;
            let rc = (s[s.len() - 1 - i] - b'a') as i64;
            l = (l * 26 + lc) % m;
            r = (rc * pow + r) % m;
            pow = pow * 26 % m;
            if l == r {
                v.push(i);
            }
        }
        for p in v.into_iter().rev() {
            if s[0..=p] == s[s.len() - 1 - p..] {
                return String::from_utf8(s[0..=p].to_vec()).unwrap();
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_prefix() {
        assert_eq!(Solution::longest_prefix("level".to_string()), "l");
        assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab");
    }
}
