// https://leetcode.com/problems/ugly-number-iii/
// 1201. Ugly Number III
pub struct Solution;
impl Solution {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        loop {
            if b == 0 {
                return a;
            }
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
        }
    }
    fn lcm(a: i64, b: i64) -> i64 {
        a * b / Solution::gcd(a, b)
    }
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        if a == 1 || b == 1 || c == 1 {
            return n;
        }
        let mut left = 1;
        let mut right = 2_000_000_000;
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let n = n as i64;
        let ab = Solution::lcm(a, b);
        let ac = Solution::lcm(a, c);
        let bc = Solution::lcm(b, c);
        let abc = Solution::lcm(ab, c);
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            let count = mid / a + mid / b + mid / c - mid / ab - mid / bc - mid / ac + mid / abc;
            if count < n {
                left = mid
            } else {
                right = mid
            }
        }
        right as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(1, 2, 3, 1), 1);
        assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
        assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
        assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
    }
}
