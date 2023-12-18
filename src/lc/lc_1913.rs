// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
// 1913. Maximum Product Difference Between Two Pairs
pub struct Solution;
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        for n in nums {
            if n > max1 {
                max2 = max1;
                max1 = n;
            } else if n > max2 {
                max2 = n;
            }
            if n < min1 {
                min2 = min1;
                min1 = n;
            } else if n < min2 {
                min2 = n;
            }
        }
        max1 * max2 - min1 * min2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_product_difference() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
    }
}
