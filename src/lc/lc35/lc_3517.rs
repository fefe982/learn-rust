// https://leetcode.com/problems/smallest-palindromic-rearrangement-i/
// 3517. Smallest Palindromic Rearrangement I
pub struct Solution;
impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let len = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut half = s.iter().take(len / 2).copied().collect::<Vec<char>>();
        half.sort_unstable();
        let mut ans = String::new();
        for &c in &half {
            ans.push(c);
        }
        if len % 2 == 1 {
            ans.push(s[len / 2]);
        }
        for &c in half.iter().rev() {
            ans.push(c);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_palindrome() {
        assert_eq!(Solution::smallest_palindrome("z".to_string()), "z".to_string());
        assert_eq!(Solution::smallest_palindrome("babab".to_string()), "abbba".to_string());
        assert_eq!(
            Solution::smallest_palindrome("daccad".to_string()),
            "acddca".to_string()
        );
    }
}
