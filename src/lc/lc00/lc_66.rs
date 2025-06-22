// https://leetcode.com/problems/plus-one/
// 66. Plus One
pub struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            let v = digits[i] + carry;
            digits[i] = v % 10;
            carry = v / 10;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
