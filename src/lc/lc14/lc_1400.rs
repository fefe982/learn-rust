// https://leetcode.com/problems/construct-k-palindrome-strings/
// 1400. Construct K Palindrome Strings
pub struct Solution;
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let mut cnt = [0; 26];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        let mut odd = 0;
        for i in 0..26 {
            if cnt[i] % 2 == 1 {
                odd += 1;
            }
        }
        odd <= k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_construct() {
        assert_eq!(Solution::can_construct("annabelle".to_string(), 2), true);
        assert_eq!(Solution::can_construct("leetcode".to_string(), 3), false);
        assert_eq!(Solution::can_construct("true".to_string(), 4), true);
        assert_eq!(Solution::can_construct("yzyzyzyzyzyzyzy".to_string(), 2), true);
        assert_eq!(Solution::can_construct("cr".to_string(), 7), false);
    }
}
