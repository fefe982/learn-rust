// https://leetcode.com/problems/reverse-bits/
// 190. Reverse Bits
pub struct Solution;
impl Solution {
    pub fn reverse_bits(x: i32) -> i32 {
        x.reverse_bits()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(2147483644), 1073741822);
    }
}
