// https://leetcode.com/problems/longest-substring-of-all-vowels-in-order/
// 1839. Longest Substring of All Vowels in Order
pub struct Solution;
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let bytes = word.as_bytes();
        if bytes.is_empty() {
            return 0;
        }

        let mut ans = 0_i32;
        let mut len = 0_i32;
        let mut groups = 0_i32;

        for i in 0..bytes.len() {
            if i == 0 || bytes[i] < bytes[i - 1] {
                if bytes[i] == b'a' {
                    len = 1;
                    groups = 1;
                } else {
                    len = 0;
                    groups = 0;
                }
            } else {
                len += 1;
                if bytes[i] > bytes[i - 1] {
                    groups += 1;
                }
            }

            if groups == 5 {
                ans = ans.max(len);
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_beautiful_substring() {
        assert_eq!(
            Solution::longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string()),
            13
        );
        assert_eq!(
            Solution::longest_beautiful_substring("aeeeiiiioooauuuaeiou".to_string()),
            5
        );
        assert_eq!(Solution::longest_beautiful_substring("a".to_string()), 0);
    }
}
