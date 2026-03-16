// https://leetcode.com/problems/longest-nice-substring/
// 1763. Longest Nice Substring
pub struct Solution;
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = (0, 0);
        for i in 0..s.len() {
            let mut lower = 0;
            let mut upper = 0;
            for j in i..s.len() {
                if s[j].is_ascii_lowercase() {
                    lower |= 1 << (s[j] as u8 - b'a');
                } else {
                    upper |= 1 << (s[j] as u8 - b'A');
                }
                if lower == upper && j - i > ans.1 - ans.0 {
                    ans = (i, j);
                }
            }
        }
        if ans.0 == ans.1 {
            return String::new();
        }
        s[ans.0..=ans.1].iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_nice_substring() {
        assert_eq!(
            Solution::longest_nice_substring("YazaAay".to_string()),
            "aAa".to_string()
        );
        assert_eq!(Solution::longest_nice_substring("Bb".to_string()), "Bb".to_string());
        assert_eq!(Solution::longest_nice_substring("c".to_string()), "".to_string());
    }
}
