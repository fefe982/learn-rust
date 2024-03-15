// https://leetcode.com/problems/product-of-array-except-self/
// 238. Product of Array Except Self
pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut p = vec![1; nums.len()];
        for (i, &n) in nums.iter().enumerate().rev().take(nums.len() - 1) {
            p[i - 1] = p[i] * n;
        }
        let mut prod = 1;
        for (i, &n) in nums.iter().enumerate().take(nums.len() - 1) {
            prod *= n;
            p[i + 1] *= prod;
        }
        p
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_except_self() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), [24, 12, 8, 6]);
        assert_eq!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }
}
