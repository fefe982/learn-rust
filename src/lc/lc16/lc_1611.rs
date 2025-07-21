// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
// 1611. Minimum One Bit Operations to Make Integers Zero
pub struct Solution;
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut mask = 1 << 30;
        let mut count = 0;
        let mut target = 0;
        while mask > 0 {
            if (n & mask != 0 && target == 0) || (n & mask == 0 && target != 0) {
                count += mask;
                target = 1;
            } else {
                target = 0;
            }
            mask >>= 1;
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_one_bit_operations() {
        assert_eq!(Solution::minimum_one_bit_operations(9), 14);
        assert_eq!(Solution::minimum_one_bit_operations(3), 2);
        assert_eq!(Solution::minimum_one_bit_operations(6), 4);
    }
}
