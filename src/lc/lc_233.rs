// https://leetcode.com/problems/number-of-digit-one/
// 233. Number of Digit One
pub struct Solution;
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut n = n;
        if n == 0 {
            return 0;
        }
        let mut id = 0;
        let mut d = 1;
        while n / d >= 10 {
            d *= 10;
            id += 1;
        }
        let mut sum = 0;
        while n > 0 {
            if n / d > 0 {
                sum += d / 10 * id * (n / d);
                if n / d > 1 {
                    sum += d;
                } else {
                    sum += n - d + 1;
                }
                n -= n / d * d;
            }
            d /= 10;
            id -= 1;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_digit_one() {
        assert_eq!(Solution::count_digit_one(110), 33);
        assert_eq!(Solution::count_digit_one(101), 23);
        assert_eq!(Solution::count_digit_one(13), 6);
        assert_eq!(Solution::count_digit_one(0), 0);
    }
}
