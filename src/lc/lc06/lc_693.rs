// https://leetcode.com/problems/binary-number-with-alternating-bits/
// 693. Binary Number with Alternating Bits
pub struct Solution;
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = n ^ (n >> 1);
        n.leading_zeros() + n.trailing_ones() == i32::BITS
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_alternating_bits() {
        assert_eq!(Solution::has_alternating_bits(5), true);
        assert_eq!(Solution::has_alternating_bits(7), false);
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}
