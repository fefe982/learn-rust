// https://leetcode.com/problems/divide-two-integers/
// 29. Divide Two Integers
pub struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        if divisor == i32::MIN {
            return if dividend == i32::MIN { 1 } else { 0 };
        }
        if dividend == 0 {
            return 0;
        }
        let sign = if (dividend < 0) ^ (divisor < 0) { -1 } else { 1 };
        let mut dvd = dividend;
        let mut dvs = divisor;
        if dvd > 0 {
            dvd = -dvd;
        }
        if dvs > 0 {
            dvs = -dvs;
        }
        let mut q = 0;
        let mut stk = vec![(dvs, 1)];
        while let Some(&(s, m)) = stk.last() {
            if dvd - s <= s {
                stk.push((s + s, m + m));
            } else {
                if dvd <= s {
                    dvd -= s;
                    q += m;
                }
                stk.pop();
            }
        }
        if sign > 0 {
            q
        } else {
            -q
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divide() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(0, 1), 0);
        assert_eq!(Solution::divide(1, 1), 1);
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
}
