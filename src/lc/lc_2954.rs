// https://leetcode.com/problems/count-the-number-of-infection-sequences/
// 2954. Count the Number of Infection Sequences
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn pow2(mut n: usize) -> i64 {
        let mut pow = 2;
        let mut res = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = res * pow % MOD;
            }
            pow = pow * pow % MOD;
            n >>= 1;
        }
        res
    }
    fn div(mut a: i64, mut b: i64) -> i64 {
        while a % b != 0 {
            let n = MOD / b + 1;
            b = b * n % MOD;
            a = a * n % MOD;
        }
        a / b
    }
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        let len = n as usize - sick.len();
        let mut fact = vec![0; len + 1];
        fact[0] = 1;
        for i in 1..=len {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let mut res = fact[len];
        res = Self::div(res, fact[sick[0] as usize]);
        res = Self::div(res, fact[n as usize - 1 - *sick.last().unwrap() as usize]);
        for i in 1..sick.len() {
            let k = sick[i] - sick[i - 1];
            if k > 2 {
                res = res * Self::pow2(k as usize - 2) % MOD;
                res = Self::div(res, fact[k as usize - 1]);
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_sequence() {
        assert_eq!(Solution::number_of_sequence(5, vec![0, 4]), 4);
        assert_eq!(Solution::number_of_sequence(4, vec![1]), 3);
    }
}
