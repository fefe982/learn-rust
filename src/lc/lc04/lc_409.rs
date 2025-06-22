// https://leetcode.com/problems/longest-palindrome/
// 409. Longest Palindrome
pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = vec![0; 52];
        for c in s.as_bytes() {
            match c {
                b'a'..=b'z' => cnt[(c - b'a') as usize] += 1,
                b'A'..=b'Z' => cnt[(c - b'A') as usize + 26] += 1,
                _ => unreachable!(),
            }
        }
        let mut sum = 0;
        let mut odd = false;
        for c in cnt {
            sum += c;
            if c % 2 == 1 {
                odd = true;
                sum -= 1;
            }
        }
        sum + if odd { 1 } else { 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }
}
