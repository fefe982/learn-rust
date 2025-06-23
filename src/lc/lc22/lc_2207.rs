// https://leetcode.com/problems/maximize-number-of-subsequences-in-a-string/
// 2207. Maximize Number of Subsequences in a String
pub struct Solution;
impl Solution {
    fn count_one(text: &[u8], pattern: u8) -> i64 {
        let cnt = text.iter().filter(|&c| *c == pattern).count() as i64;
        cnt * (cnt + 1) / 2
    }
    fn count_two(text: &[u8], pattern: &[u8]) -> i64 {
        let mut c0 = 0;
        let mut c1 = 0;
        let mut ans = 0;
        for i in 0..text.len() {
            if text[i] == pattern[0] {
                c0 += 1;
            } else if text[i] == pattern[1] {
                c1 += 1;
                ans += c0;
            }
        }
        ans + c0.max(c1)
    }
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let text = text.as_bytes();
        let pattern = pattern.as_bytes();
        if pattern[0] == pattern[1] {
            Self::count_one(text, pattern[0])
        } else {
            Self::count_two(text, pattern)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_subsequence_count() {
        assert_eq!(
            Solution::maximum_subsequence_count("abdcdbc".to_string(), "ac".to_string()),
            4
        );
        assert_eq!(
            Solution::maximum_subsequence_count("aabb".to_string(), "ab".to_string()),
            6
        );
    }
}
