// https://leetcode.com/problems/palindrome-number/
// 9. Palindrome Number
pub struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if 0 <= x && x < 10 {
            return true;
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut x = x;
        let mut rev = 0;
        while x > rev {
            rev = rev * 10 + x % 10;
            x /= 10;
            if x == rev || rev == x / 10 {
                return true;
            }
        }
        x == rev
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
