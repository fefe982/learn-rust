// https://leetcode.com/problems/sign-of-the-product-of-an-array/
// 1822. Sign of the Product of an Array
pub struct Solution;
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for n in nums {
            if n < 0 {
                sign = -sign;
            } else if n == 0 {
                return 0;
            }
        }
        sign
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_sign() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
