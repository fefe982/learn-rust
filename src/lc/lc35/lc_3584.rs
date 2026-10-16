// https://leetcode.com/problems/maximum-product-of-first-and-last-elements-of-a-subsequence/
// 3584. Maximum Product of First and Last Elements of a Subsequence
pub struct Solution;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        if m == 1 {
            let max = nums.into_iter().map(|i| i.abs()).max().unwrap();
            return max as i64 * max as i64;
        }
        let mut mul = i64::MIN;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let m = m as usize;
        for i in 0..nums.len() - m + 1 {
            min = min.min(nums[i]);
            max = max.max(nums[i]);
            if nums[i + m - 1] < 0 {
                mul = mul.max(min as i64 * nums[i + m - 1] as i64);
            } else {
                mul = mul.max(max as i64 * nums[i + m - 1] as i64);
            }
        }
        mul
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_product() {
        assert_eq!(Solution::maximum_product(vec![-1, -9, 2, 3, -2, -3, 1], 1), 81);
        assert_eq!(Solution::maximum_product(vec![1, 3, -5, 5, 6, -4], 3), 20);
        assert_eq!(Solution::maximum_product(vec![2, -1, 2, -6, 5, 2, -5, 7], 2), 35);
    }
}
