// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
// 1464. Maximum Product of Two Elements in an Array
pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (m0, m1) = nums.into_iter().fold((0, 0), |(m0, m1), x| {
            if x >= m1 {
                (m1, x)
            } else if x >= m0 {
                (x, m1)
            } else {
                (m0, m1)
            }
        });
        (m0 - 1) * (m1 - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }
}
