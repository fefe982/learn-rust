// https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/
// 2730. Find the Longest Semi-Repetitive Substring
pub struct Solution;
impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let s = s.as_bytes();
        if s.len() == 1 {
            return 1;
        }
        let mut ans = 2;
        let mut j = 0;
        let mut c = 0;
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                c += 1;
            }
            while c > 1 {
                if s[j] == s[j + 1] {
                    c -= 1;
                }
                j += 1;
            }
            ans = ans.max(i - j + 1);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_semi_repetitive_substring() {
        assert_eq!(Solution::longest_semi_repetitive_substring("52233".to_string()), 4);
        assert_eq!(Solution::longest_semi_repetitive_substring("5494".to_string()), 4);
        assert_eq!(Solution::longest_semi_repetitive_substring("1111111".to_string()), 2);
    }
}
