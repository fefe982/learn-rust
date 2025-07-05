// https://leetcode.com/problems/maximize-number-of-nice-divisors/
// 1808. Maximize Number of Nice Divisors
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn pow(a: i32, b: i32) -> i64 {
        let mut a = a as i64;
        let mut b = b as i64;
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = (res * a) % MOD;
            }
            a = (a * a) % MOD;
            b >>= 1;
        }
        res
    }
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        if prime_factors < 5 {
            return prime_factors;
        }
        let (n2, n3) = match prime_factors % 3 {
            0 => (1, prime_factors / 3),
            1 => (4, (prime_factors - 4) / 3),
            2 => (2, (prime_factors - 2) / 3),
            _ => unreachable!(),
        };
        return (n2 * Self::pow(3, n3) % MOD) as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_nice_divisors() {
        assert_eq!(Solution::max_nice_divisors(5), 6);
        assert_eq!(Solution::max_nice_divisors(8), 18);
    }
}
