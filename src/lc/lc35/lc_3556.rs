// https://leetcode.com/problems/sum-of-largest-prime-substrings/
// 3556. Sum of Largest Prime Substrings
pub struct Solution;
impl Solution {
    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i64) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn sum_of_largest_primes(s: String) -> i64 {
        let s = s.as_bytes();
        let mut primes = vec![0, 0, 0];
        for i in 0..s.len() {
            let mut n = 0;
            for j in i..s.len() {
                n = n * 10 + (s[j] - b'0') as i64;
                if n <= primes[2] || n == primes[1] || n == primes[0] || !Self::is_prime(n) {
                    continue;
                }
                if n < primes[1] {
                    primes[2] = n;
                } else {
                    primes[2] = primes[1];
                    if n < primes[0] {
                        primes[1] = n;
                    } else {
                        primes[1] = primes[0];
                        primes[0] = n;
                    }
                }
            }
        }
        primes[0] + primes[1] + primes[2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_largest_primes() {
        assert_eq!(Solution::sum_of_largest_primes("12234".to_string()), 1469);
        assert_eq!(Solution::sum_of_largest_primes("111".to_string()), 11);
    }
}
