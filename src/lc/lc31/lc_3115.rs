// https://leetcode.com/problems/maximum-prime-difference/
// 3115. Maximum Prime Difference
pub struct Solution;
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let primes = vec![
            false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false,
            false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true,
            false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true,
            false, false, false, false, false, true, false, false, false, false, false, true, false, true, false,
            false, false, false, false, true, false, false, false, true, false, true, false, false, false, false,
            false, true, false, false, false, true, false, false, false, false, false, true, false, false, false,
            false, false, false, false, true, false, false, false,
        ];
        (nums.len()
            - nums.iter().position(|&x| primes[x as usize]).unwrap()
            - nums.iter().rev().position(|&x| primes[x as usize]).unwrap()
            - 1) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_prime_difference() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
        assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    }
}
