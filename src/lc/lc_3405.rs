// https://leetcode.com/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/
// 3405. Count the Number of Arrays with K Matching Adjacent Elements
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn p(n: i64, k: i64) -> i64 {
        let mut k = k;
        let mut r = 1;
        let mut n = n;
        while k > 0 {
            if k & 1 == 1 {
                r = (r * n) % MOD;
            }
            n = (n * n) % MOD;
            k >>= 1;
        }
        r
    }
    fn c(n: i64, k: i64) -> i64 {
        let mut k = k;
        if n - k < k {
            k = n - k;
        }
        let mut u = 1;
        let mut d = 1;
        for i in 1..=k {
            u = (u * (n - i + 1)) % MOD;
            d = (d * i) % MOD;
        }
        while u % d != 0 {
            let t = MOD / d + 1;
            u = (u * t) % MOD;
            d = (d * t) % MOD;
        }
        u / d
    }
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;
        (m * Solution::p(m - 1, n - 1 - k) % MOD * Solution::c(n - 1, k) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_arrays() {
        assert_eq!(Solution::count_good_arrays(42367, 39220, 7691), 512432464);
        assert_eq!(Solution::count_good_arrays(40603, 16984, 29979), 235077659);
        assert_eq!(Solution::count_good_arrays(3, 2, 1), 4);
        assert_eq!(Solution::count_good_arrays(4, 2, 2), 6);
        assert_eq!(Solution::count_good_arrays(5, 2, 0), 2);
    }
}
