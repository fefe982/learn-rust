// https://leetcode.cn/problems/valid-palindrome-ii/
// 680. Valid Palindrome II
pub struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j && s[i] == s[j] {
            i += 1;
            j -= 1;
        }
        if i >= j {
            return true;
        }
        let mut i1 = i;
        let mut j1 = j - 1;
        while i1 < j1 && s[i1] == s[j1] {
            i1 += 1;
            j1 -= 1;
        }
        if i1 >= j1 {
            return true;
        }
        i1 = i + 1;
        j1 = j;
        while i1 < j1 && s[i1] == s[j1] {
            i1 += 1;
            j1 -= 1;
        }
        i1 >= j1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_palindrome() {
        assert_eq!(Solution::valid_palindrome("eceec".to_string()), true);
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
    }
}
