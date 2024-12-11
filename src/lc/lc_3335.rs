// https://leetcode.com/problems/total-characters-in-string-after-transformations-i/
// 3335. Total Characters in String After Transformations I
pub struct Solution;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let t = t as usize;
        let mut cnt = vec![0; 26 + t];
        let m = 1_000_000_007;
        for i in 0..26 {
            cnt[i] = 1;
        }
        for i in 0..t {
            cnt[26 + i] = (cnt[i] + cnt[i + 1]) % m;
        }
        let mut res = 0;
        for c in s.as_bytes() {
            res = (res + cnt[(c - b'a') as usize + t]) % m
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_after_transformations() {
        assert_eq!(Solution::length_after_transformations("abcyy".to_string(), 2), 7);
        assert_eq!(Solution::length_after_transformations("azbk".to_string(), 1), 5);
    }
}
