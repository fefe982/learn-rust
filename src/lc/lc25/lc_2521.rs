// https://leetcode.com/problems/distinct-prime-factors-of-product-of-array/
// 2521. Distinct Prime Factors of Product of Array
pub struct Solution;
impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let mut prime = vec![false; 1001];
        let mut vp = vec![];
        for i in 2..=1000 {
            prime[i] = true;
            vp.push(i);
            for j in (i * i..=1000).step_by(i as usize) {
                prime[j] = false;
            }
        }
        let mut res = 0;
        for &num in &nums {
            let mut n = num as usize;
            for &p in &vp {
                if n % p == 0 {
                    if prime[p] {
                        res += 1;
                        prime[p] = false;
                    }
                    while n % p == 0 {
                        n /= p;
                    }
                }
                if n == 1 {
                    break;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinct_prime_factors() {
        assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]), 4);
        assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 8, 16]), 1);
    }
}
