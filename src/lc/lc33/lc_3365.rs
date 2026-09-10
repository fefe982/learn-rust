// https://leetcode.com/problems/rearrange-k-substrings-to-form-target-string/
// 3365. Rearrange K Substrings to Form Target String
pub struct Solution;
impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let mut ms = std::collections::HashMap::new();
        let mut mt = std::collections::HashMap::new();
        let k = k as usize;
        let s = s.as_bytes();
        let t = t.as_bytes();
        let l = s.len() / k;
        for i in 0..k {
            *ms.entry(&s[i * l..(i + 1) * l]).or_insert(0) += 1;
            *mt.entry(&t[i * l..(i + 1) * l]).or_insert(0) += 1;
        }
        ms == mt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_possible_to_rearrange() {
        assert_eq!(
            Solution::is_possible_to_rearrange("abcd".to_string(), "cdab".to_string(), 2),
            true
        );
        assert_eq!(
            Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 3),
            true
        );
        assert_eq!(
            Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 2),
            false
        );
    }
}
