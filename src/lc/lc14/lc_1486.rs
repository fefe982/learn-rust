// https://leetcode.com/problems/xor-operation-in-an-array/
// 1486. XOR Operation in an Array
pub struct Solution;
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xor_operation() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
    }
}
