// https://leetcode.com/problems/max-dot-product-of-two-subsequences/
// 1458. Max Dot Product of Two Subsequences
pub struct Solution;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut s = vec![0; nums1.len()];
        s[0] = nums1[0] * nums2[0];
        for i in 1..nums1.len() {
            s[i] = (nums1[i] * nums2[0]).max(s[i - 1]);
        }
        for j in 1..nums2.len() {
            let mut t = s.clone();
            t[0] = s[0].max(nums1[0] * nums2[j]);
            for i in 1..nums1.len() {
                t[i] = s[i]
                    .max(t[i - 1])
                    .max(s[i - 1].max(0) + nums1[i] * nums2[j]);
            }
            s = t;
        }
        s[nums1.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_dot_product() {
        assert_eq!(
            Solution::max_dot_product(
                vec![-3, -8, 3, -10, 1, 3, 9],
                vec![9, 2, 3, 7, -9, 1, -8, 5, -1, -1]
            ),
            200
        );
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
    }
}
