// https://leetcode.com/problems/monotone-increasing-digits/
// 738. Monotone Increasing Digits
pub struct Solution;
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut last = n % 10;
        let mut m = 10;
        while m <= n {
            let d = (n / m) % 10;
            if d > last {
                n = n / m * m - 1;
                last = d - 1;
            } else {
                last = d;
            }
            m *= 10;
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monotone_increasing_digits() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
}
