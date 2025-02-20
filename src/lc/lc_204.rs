// https://leetcode.com/problems/count-primes/
// 204. Count Primes
pub struct Solution;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut is_prime = vec![true; n as usize];
        let mut count = 0;
        let mut i = 2;
        loop {
            if is_prime[i as usize] {
                count += 1;
                let mut j = i * i;
                if j < n {
                    while j < n {
                        is_prime[j as usize] = false;
                        j += i;
                    }
                } else {
                    i += 1;
                    break;
                }
            }
            i += 1;
        }
        while i < n {
            if is_prime[i as usize] {
                count += 1;
            }
            i += 1;
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_primes() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(2), 0);
    }
}
