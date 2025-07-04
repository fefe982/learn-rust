// https://leetcode.com/problems/distinct-echo-substrings/
// 1316. Distinct Echo Substrings
pub struct Solution;
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let text = text.as_bytes();
        let mut ans = 0;
        for i in 1..=text.len() / 2 {
            let mut cnt = 0;
            let mut set = std::collections::HashSet::new();
            for s in 0..text.len() - i {
                if text[s] == text[s + i] {
                    cnt += 1;
                } else {
                    cnt = 0;
                }
                if cnt >= i {
                    if !set.contains(&text[s + 1 - i..s + 1]) {
                        ans += 1;
                        set.insert(&text[s + 1 - i..s + 1]);
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distinct_echo_substrings() {
        assert_eq!(Solution::distinct_echo_substrings(String::from("abcabcabc")), 3);
        assert_eq!(Solution::distinct_echo_substrings(String::from("leetcodeleetcode")), 2);
    }
}
