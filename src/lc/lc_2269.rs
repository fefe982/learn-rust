// https://leetcode.com/problems/find-the-k-beauty-of-a-number/
// 2269. Find the Kth Beauty of a Number
pub struct Solution;
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut n = num;
        let mut div = 0;
        let mut mul = 1;
        for _ in 0..k - 1 {
            let d = n % 10;
            n /= 10;
            div = div + d * mul;
            mul *= 10;
        }
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            div += d * mul;
            if div != 0 && num % div == 0 {
                res += 1;
            }
            div /= 10;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divisor_substrings() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    }
}
