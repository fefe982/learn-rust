// https://leetcode.com/problems/wildcard-matching/
// 44. Wildcard Matching
pub struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut is = 0;
        let mut ip = 0;
        let mut s_star = usize::MAX;
        let mut p_star = usize::MAX;
        while is < s.len() {
            if ip < p.len() && (p[ip] == b'?' || p[ip] == s[is]) {
                is += 1;
                ip += 1;
            } else if ip < p.len() && p[ip] == b'*' {
                s_star = is;
                p_star = ip;
                ip += 1;
            } else if p_star != usize::MAX {
                ip = p_star + 1;
                s_star += 1;
                is = s_star
            } else {
                return false;
            }
        }
        while ip < p.len() && p[ip] == b'*' {
            ip += 1;
        }
        ip == p.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_match() {
        assert_eq!(Solution::is_match(String::from(""), String::from("****")), true);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("*")), true);
        assert_eq!(Solution::is_match(String::from("cb"), String::from("?a")), false);
    }
}
