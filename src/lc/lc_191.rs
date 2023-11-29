// https://leetcode.com/problems/number-of-1-bits/
// 191. Number of 1 Bits
pub struct Solution;
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hamming_weight() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
        assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
