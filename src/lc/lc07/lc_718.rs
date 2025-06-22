// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
// 718. Maximum Length of Repeated Subarray
pub struct Solution;
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums2.len() + 1];
        let mut max = 0;
        for i in 0..nums1.len() {
            let mut ndp = dp.clone();
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    ndp[j + 1] = dp[j] + 1;
                    max = max.max(ndp[j + 1]);
                } else {
                    ndp[j + 1] = 0;
                }
            }
            dp = ndp;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_length() {
        assert_eq!(Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
        assert_eq!(Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]), 5);
    }
}
