// https://leetcode.com/problems/sum-of-two-integers/
// 371. Sum of Two Integers
pub struct Solution;
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let carry = (a & b) << 1;
            a = a ^ b;
            b = carry;
        }
        a
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_sum() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(2, 3), 5);
    }
}
