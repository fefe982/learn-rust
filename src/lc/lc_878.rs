// https://leetcode.com/problems/nth-magical-number/
// 878. Nth Magical Number
pub struct Solution;
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let md = 1000000007;
        let n = n as i64;
        let (a, b) = (a.min(b) as i64, a.max(b) as i64);
        let mut aa = a;
        let mut bb = b;
        let gcd = loop {
            if aa == 0 {
                break bb;
            }
            bb = bb % aa;
            if bb == 0 {
                break aa;
            }
            aa = aa % bb;
        };
        let mut min = a;
        let mut max = a * n;
        let f = |x: i64| x / a + x / b - x / (a * b / gcd);
        if f(max) == n {
            return (max % md) as i32;
        }
        while min + 1 < max {
            let mid = (min + max) / 2;
            if f(mid) < n {
                min = mid;
            } else {
                max = mid;
            }
        }
        (max % md) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nth_magical_number() {
        assert_eq!(Solution::nth_magical_number(5, 2, 4), 10);
        assert_eq!(Solution::nth_magical_number(1, 2, 3), 2);
        assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
    }
}
