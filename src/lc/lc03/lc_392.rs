// https://leetcode.com/problems/is-subsequence/
// 392. Is Subsequence
pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut s, mut t) = (s.chars(), t.chars());
        loop {
            match (s.next(), t.next()) {
                (Some(c), Some(cc)) => {
                    let mut ct = cc;
                    while ct != c {
                        if let Some(cn) = t.next() {
                            ct = cn;
                        } else {
                            return false;
                        }
                    }
                }
                (Some(_), None) => {
                    return false;
                }
                (None, _) => {
                    return true;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
