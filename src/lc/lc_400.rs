// https://leetcode.com/problems/nth-digit/
// 400. Nth Digit
pub struct Solution;
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut i = 9i32;
        let mut d = 1;
        let mut n = n;
        loop {
            if n > i.saturating_mul(d) {
                n -= i * d;
                i = i * 10;
                d += 1;
            } else {
                n = n - 1;
                let s = i / 9;
                let j = n / d;
                let mut k = d - n % d;
                n = s + j;
                while k > 1 {
                    n /= 10;
                    k -= 1;
                }
                return n % 10;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_nth_digit() {
        assert_eq!(Solution::find_nth_digit(3), 3);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(100), 5);
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
    }
}
