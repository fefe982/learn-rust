// https://leetcode.com/problems/minimize-xor/
// 2429. Minimize XOR
pub struct Solution;
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let nbit2 = num2.count_ones() as i32;
        let nbit1 = num1.count_ones() as i32;
        let mut one_mask = num1;
        for _ in 0..nbit1 - nbit2 {
            one_mask &= one_mask - 1;
        }
        let mut zero_mask = !num1;
        for _ in 0..nbit2 - nbit1 {
            zero_mask &= zero_mask - 1;
        }
        zero_mask = !(zero_mask | num1);
        one_mask | zero_mask
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimize_xor() {
        assert_eq!(Solution::minimize_xor(3, 5), 3);
        assert_eq!(Solution::minimize_xor(1, 12), 3);
    }
}
