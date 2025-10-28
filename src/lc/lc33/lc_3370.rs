// https://leetcode.com/problems/smallest-number-with-all-set-bits/
// 3370. Smallest Number with All Set Bits
pub struct Solution;
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        (1 << (i32::BITS - n.leading_zeros())) - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_number() {
        assert_eq!(Solution::smallest_number(5), 7);
        assert_eq!(Solution::smallest_number(10), 15);
        assert_eq!(Solution::smallest_number(3), 3);
    }
}
