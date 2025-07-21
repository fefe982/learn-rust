// https://leetcode.com/problems/split-two-strings-to-make-palindrome/
// 1616. Split Two Strings to Make Palindrome
pub struct Solution;
impl Solution {
    fn check_one_slice(x: &[u8]) -> bool {
        let l = x.len();
        for i in 0..(l / 2) {
            if x[i] != x[l - i - 1] {
                return false;
            }
        }
        true
    }
    fn check_slice(a: &[u8], b: &[u8]) -> bool {
        if a.len() <= 1 {
            return true;
        }
        Self::check_one_slice(a)
            || Self::check_one_slice(b)
            || (a[0] == b[b.len() - 1]
                && Self::check_slice(&a[1..(a.len() - 1)], &b[1..(b.len() - 1)]))
    }
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        Self::check_slice(a.as_bytes(), b.as_bytes())
            || Self::check_slice(b.as_bytes(), a.as_bytes())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_palindrome_formation() {
        // assert_eq!(
        //     Solution::check_palindrome_formation(String::from("x"), String::from("y")),
        //     true
        // );
        assert_eq!(
            Solution::check_palindrome_formation(String::from("xbdef"), String::from("xecab")),
            false
        );
        assert_eq!(
            Solution::check_palindrome_formation(String::from("ulacfd"), String::from("jizalu")),
            true
        );
    }
}
