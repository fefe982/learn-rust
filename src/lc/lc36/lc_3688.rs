// https://leetcode.com/problems/bitwise-or-of-even-numbers-in-an-array/
// 3688. Bitwise OR of Even Numbers in an Array
pub struct Solution;
impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for &num in &nums {
            if num % 2 == 0 {
                result |= num;
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn even_number_bitwise_o_rs() {
        assert_eq!(Solution::even_number_bitwise_o_rs(vec![1, 2, 3, 4, 5, 6]), 6);
        assert_eq!(Solution::even_number_bitwise_o_rs(vec![7, 9, 11]), 0);
        assert_eq!(Solution::even_number_bitwise_o_rs(vec![1, 8, 16]), 24);
    }
}
