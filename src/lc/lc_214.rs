// https://leetcode.com/problems/shortest-palindrome/
// 214. Shortest Palindrome
pub struct Solution;
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let s = s.as_bytes();
        let mut p_idx = 0;
        for end in (0..s.len()).rev() {
            let mut good = true;
            for idx in 0..(end + 1) / 2 {
                if s[idx] != s[end - idx] {
                    good = false;
                    break;
                }
            }
            if good {
                p_idx = end;
                break;
            }
        }
        String::from_utf8(
            s[p_idx + 1..]
                .iter()
                .rev()
                .chain(s.iter())
                .cloned()
                .collect(),
        )
        .unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_palindrome() {
        assert_eq!(
            Solution::shortest_palindrome(String::from("")),
            String::from("")
        );
        assert_eq!(
            Solution::shortest_palindrome(String::from("aacecaaa")),
            String::from("aaacecaaa")
        );
        assert_eq!(
            Solution::shortest_palindrome(String::from("abcd")),
            String::from("dcbabcd")
        );
    }
}
