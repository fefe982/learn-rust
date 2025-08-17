// https://leetcode.com/problems/complement-of-base-10-integer/
// 1009. Complement of Base 10 Integer
pub struct Solution;
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            ((1 << (32 - n.leading_zeros())) - 1) & (!n)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bitwise_complement() {
        assert_eq!(Solution::bitwise_complement(5), 2);
        assert_eq!(Solution::bitwise_complement(7), 0);
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}
