// https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/
// 2507. Smallest Value After Replacing With Sum of Prime Factors
pub struct Solution;
impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        let mut n = n;
        loop {
            if n == 4 {
                return 4;
            }
            let mut sum = 0;
            let mut i = 2;
            while i * i <= n {
                while n % i == 0 {
                    sum += i;
                    n /= i;
                }
                i += 1;
            }
            if n > 1 {
                if sum == 0 {
                    return n;
                }
                sum += n;
            }
            n = sum;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_value() {
        assert_eq!(Solution::smallest_value(4), 4);
        assert_eq!(Solution::smallest_value(15), 5);
        assert_eq!(Solution::smallest_value(3), 3);
    }
}
