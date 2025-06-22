// https://leetcode.com/problems/powx-n/
// 50. Pow(x, n)
pub struct Solution;
impl Solution {
    fn pow_i64(x: f64, n: i64) -> f64 {
        let mut res = 1.0;
        for i in (0..32).rev() {
            res = res * res;
            if n & (1 << i) != 0 {
                res *= x;
            }
        }
        res
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1.0
        } else if x == 0.0 {
            0.0
        } else if n > 0 {
            Self::pow_i64(x, n as i64)
        } else {
            1.0 / Self::pow_i64(x, -(n as i64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn my_pow() {
        assert_approx_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_approx_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_approx_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
