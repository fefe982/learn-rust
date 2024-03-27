// https://leetcode.com/problems/subarray-product-less-than-k/
// 713. Subarray Product Less Than K
pub struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut ans = 0;
        let mut i = 0;
        let mut j = 1;
        let mut prod = nums[0];
        while i < nums.len() {
            while prod < k && j < nums.len() {
                prod *= nums[j];
                j += 1;
            }
            ans += (j - i - if prod < k { 0 } else { 1 }) as i32;
            prod /= nums[i];
            i += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_subarray_product_less_than_k() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
    }
}
