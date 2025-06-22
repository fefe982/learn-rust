// https://leetcode.com/problems/bitwise-and-of-numbers-range/
// 201. Bitwise AND of Numbers Range
pub struct Solution;
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut res = left as i64;
        let mut n = left as i64;
        let mut pow = 1;
        let right = right as i64;
        while n <= right {
            res &= n;
            if res == 0 {
                break;
            }
            while n & pow == 0 {
                pow <<= 1;
            }
            n += pow;
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(2, 4), 0);
        assert_eq!(Solution::range_bitwise_and(1, 2), 0);
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
}
