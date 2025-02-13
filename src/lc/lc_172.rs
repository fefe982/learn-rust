// https://leetcode.com/problems/factorial-trailing-zeroes/
// 172. Factorial Trailing Zeroes
pub struct Solution;
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut c = 0;
        while n > 0 {
            n /= 5;
            c += n;
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
