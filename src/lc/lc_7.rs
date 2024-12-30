// https://leetcode.com/problems/reverse-integer/
// 7. Reverse Integer
pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y: i32 = 0;
        while x != 0 {
            let d = x % 10;
            match y.checked_mul(10).and_then(|a| a.checked_add(d)) {
                None => return 0,
                Some(a) => y = a,
            }
            x /= 10;
        }
        y
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
