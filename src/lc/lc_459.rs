// https://leetcode.com/problems/repeated-substring-pattern/
// 459. Repeated Substring Pattern
pub struct Solution;
impl Solution {
    fn check_str(s: &[u8], n: usize) -> bool {
        let l = s.len();
        let s0 = &s[0..n];
        for i in 1..l / n {
            if s0 != &s[n * i..n * (i + 1)] {
                println!("{s:?}, {n}, {i}, {s0:?}, {:?}", &s[n * i..n * (i + 1)]);
                return false;
            }
        }
        return true;
    }
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        if s.len() < 2 {
            return false;
        }
        if Self::check_str(s, 1) {
            return true;
        }
        let mut i = 2;
        let l = s.len();
        while i * i <= l {
            if l % i == 0 {
                if Self::check_str(s, i) {
                    return true;
                }
                if l / i != i && Self::check_str(s, l / i) {
                    return true;
                }
            }
            i += 1;
        }
        return false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_substring_pattern() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from(
                "ababababababaababababababaababababababa"
            )),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abab")),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("aba")),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abcabcabc")),
            true
        );
    }
}
