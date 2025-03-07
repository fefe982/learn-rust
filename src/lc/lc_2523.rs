// https://leetcode.com/problems/closest-prime-numbers-in-range/
// 2523. Closest Prime Numbers in Range
pub struct Solution;
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        if right < 3 {
            return vec![-1, -1];
        }
        let mut primes = vec![true; (right as usize + 1) / 2];
        let mut last_prime = 2;
        let mut min = vec![-1, -1];
        for i in 1..primes.len() {
            let p = i as i32 * 2 + 1;
            if !primes[i] {
                continue;
            }
            if last_prime >= left {
                if min[0] == -1 || p - last_prime < min[1] - min[0] {
                    min = vec![last_prime, p];
                }
            }
            let mut j = (p * p / 2) as usize;
            while j < primes.len() {
                primes[j] = false;
                j += p as usize;
            }
            last_prime = p;
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closest_primes() {
        let res = Solution::closest_primes(10, 20);
        assert_eq!(res, vec![11, 13]);
        let res = Solution::closest_primes(1, 10);
        assert_eq!(res, vec![2, 3]);
    }
}
