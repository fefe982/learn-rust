// https://leetcode.com/problems/sum-of-k-digit-numbers-in-a-range/
// 3855. Sum of K-Digit Numbers in a Range
pub struct Solution;
impl Solution {
    pub fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
        let n = (r - l + 1) as i64;
        let sum = (l + r) as i64 * n / 2;
        const MOD: i64 = 1000000007;
        let mut k = k;
        let mut p10 = 1;
        let mut pn = 1;
        let mut p20 = 10;
        let mut p2n = n;
        while k > 0 {
            if k & 1 == 1 {
                p10 = (p10 * p20) % MOD;
                pn = (pn * p2n) % MOD;
            }
            p20 = (p20 * p20) % MOD;
            p2n = (p2n * p2n) % MOD;
            k >>= 1;
        }
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let t = MOD / b + 1;
                b = (b * t) % MOD;
                a = (a * t) % MOD;
            }
            a / b
        };
        p10 = div((p10 + MOD - 1) % MOD, 9);
        pn = div(pn, n);
        ((p10 * sum % MOD * pn) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_numbers() {
        assert_eq!(Solution::sum_of_numbers(9, 9, 52246276), 425168386);
        assert_eq!(Solution::sum_of_numbers(1, 2, 2), 66);
        assert_eq!(Solution::sum_of_numbers(0, 1, 3), 444);
        assert_eq!(Solution::sum_of_numbers(5, 5, 10), 555555520);
    }
}
