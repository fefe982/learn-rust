// https://leetcode.com/problems/number-of-even-and-odd-bits/
// 2595. Number of Even and Odd Bits
pub struct Solution;
impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let all = n.count_ones() as i32;
        let even = (n & 0x55555555).count_ones() as i32;
        vec![even, all - even]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even_odd_bit() {
        assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
        assert_eq!(Solution::even_odd_bit(50), vec![1, 2]);
    }
}
