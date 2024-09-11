// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/
// 2220. Minimum Bit Flips to Convert Number
pub struct Solution;
impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_bit_flips() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
