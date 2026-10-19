// https://leetcode.com/problems/check-if-any-element-has-prime-frequency/
// 3591. Check if Any Element Has Prime Frequency
pub struct Solution;
impl Solution {
    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for (_, v) in map {
            if Self::is_prime(v) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_prime_frequency() {
        assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]), true);
        assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]), false);
        assert_eq!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]), true);
    }
}
