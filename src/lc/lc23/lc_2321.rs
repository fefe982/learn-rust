// https://leetcode.com/problems/maximum-score-of-spliced-array/
// 2321. Maximum Score Of Spliced Array
pub struct Solution;
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut inc1 = 0;
        let mut inc2 = 0;
        let mut max1 = 0;
        let mut max2 = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        nums1.into_iter().zip(nums2).for_each(|(a, b)| {
            sum1 += a;
            sum2 += b;
            max1 = max1.max(0) + b - a;
            max2 = max2.max(0) + a - b;
            inc1 = inc1.max(max1);
            inc2 = inc2.max(max2);
        });
        (sum1 + inc1).max(sum2 + inc2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_spliced_array() {
        assert_eq!(
            Solution::maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
            210
        );
        assert_eq!(
            Solution::maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
            220
        );
        assert_eq!(Solution::maximums_spliced_array(vec![7, 11, 13], vec![1, 1, 1]), 31);
    }
}
