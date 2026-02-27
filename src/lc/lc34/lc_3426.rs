// https://leetcode.com/problems/manhattan-distances-of-all-arrangements-of-pieces/
// 3426. Manhattan Distances of All Arrangements of Pieces
pub struct Solution;
impl Solution {
    pub fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let t = MOD / b + 1;
                b = b * t % MOD;
                a = a * t % MOD;
            }
            a / b
        };
        let mut ans = 1;
        let m = m as i64;
        let n = n as i64;
        let k = k as i64;
        let mn = m * n - 2;
        let mut kk = k - 2;
        if mn - kk < kk {
            kk = mn - kk;
        }
        for i in 1..=kk {
            ans = div(ans * (mn - i + 1) % MOD, i);
        }
        let factor = ((m * m) % MOD * (((n + 1) * n * (n - 1) / 6) % MOD)
            + ((n * n) % MOD * (((m + 1) * m * (m - 1) / 6) % MOD)))
            % MOD;
        ((ans * factor) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance_sum() {
        assert_eq!(Solution::distance_sum(2, 2, 2), 8);
        assert_eq!(Solution::distance_sum(1, 4, 3), 20);
    }
}
