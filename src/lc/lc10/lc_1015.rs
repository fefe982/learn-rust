// https://leetcode.com/problems/smallest-integer-divisible-by-k/
// 1015. Smallest Integer Divisible by K
pub struct Solution;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        let mut n = 1;
        let mut l = 1;
        while n % k != 0 {
            n = (n * 10 + 1) % k;
            l += 1;
        }
        l
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_repunit_div_by_k() {
        assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
        assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
        assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
    }
}
