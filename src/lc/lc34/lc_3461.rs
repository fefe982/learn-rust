// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/
// 3461. Check if Digits of a Number are in Increasing Order
pub struct Solution;
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let s = s.as_bytes();
        let len = s.len();
        let mut d1 = (s[0] - b'0') as i32;
        let mut d2 = (s[1] - b'0') as i32;
        let mut p2 = 0;
        let mut p5 = 0;
        let mut r5 = 1;
        for i in 2..len {
            let mut n = (len - i) as i32;
            let mut j = (i - 1) as i32;
            p2 += n.trailing_zeros();
            p2 -= j.trailing_zeros();
            while n % 5 == 0 {
                n /= 5;
                p5 += 1;
            }
            while j % 5 == 0 {
                j /= 5;
                p5 -= 1;
            }
            j = j % 5;
            if j == 2 || j == 3 {
                j = 5 - j;
            }
            r5 = (r5 * n * j) % 5;
            let mut d = if p5 > 0 { 0 } else { (r5 * 6) % 10 };
            if p2 == 0 {
                d = (d + 5) % 10;
            }
            d1 = (d1 + (s[i - 1] - b'0') as i32 * d) % 10;
            d2 = (d2 + (s[i] - b'0') as i32 * d) % 10;
        }
        d1 == d2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_same_digits() {
        assert_eq!(Solution::has_same_digits("3902".to_string()), true);
        assert_eq!(Solution::has_same_digits("34789".to_string()), false);
    }
}
