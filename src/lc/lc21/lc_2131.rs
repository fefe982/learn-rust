// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
// 2131. Longest Palindrome by Concatenating Two Letter Words
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut cnt = vec![0; 26 * 26];
        let mut len = 0;
        let mut mid = 0;
        for word in words {
            let w = word.as_bytes();
            let iw = (w[0] - b'a') as usize * 26 + (w[1] - b'a') as usize;
            let piw = (w[1] - b'a') as usize * 26 + (w[0] - b'a') as usize;
            if iw == piw {
                if cnt[iw] == 0 {
                    mid += 1;
                    cnt[iw] += 1;
                } else {
                    len += 4;
                    cnt[iw] -= 1;
                    mid -= 1;
                }
            } else {
                if cnt[piw] > 0 {
                    len += 4;
                    cnt[piw] -= 1;
                } else {
                    cnt[iw] += 1;
                }
            }
        }
        if mid == 0 {
            len
        } else {
            len + 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome(vec_str![
                "dd", "aa", "bb", "dd", "aa", "dd", "bb", "dd", "aa", "cc", "bb", "cc", "dd", "cc"
            ]),
            22
        );
        assert_eq!(Solution::longest_palindrome(vec_str!["lc", "cl", "gg"]), 6);
        assert_eq!(
            Solution::longest_palindrome(vec_str!["ab", "ty", "yt", "lc", "cl", "ab"]),
            8
        );
        assert_eq!(Solution::longest_palindrome(vec_str!["cc", "ll", "xx"]), 2);
    }
}
