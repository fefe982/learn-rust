// https://leetcode.com/problems/get-the-maximum-score/
// 1537. Get the Maximum Score
pub struct Solution;
impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                sum1 += nums1[i] as i64;
                i += 1;
            } else if nums1[i] > nums2[j] {
                sum2 += nums2[j] as i64;
                j += 1;
            } else {
                let sum = sum1.max(sum2);
                sum1 = sum + nums1[i] as i64;
                sum2 = sum + nums2[j] as i64;
                i += 1;
                j += 1;
            }
        }
        while i < nums1.len() {
            sum1 += nums1[i] as i64;
            i += 1;
        }
        while j < nums2.len() {
            sum2 += nums2[j] as i64;
            j += 1;
        }
        (sum1.max(sum2) % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum() {
        assert_eq!(Solution::max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30);
        assert_eq!(Solution::max_sum(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
        assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 40);
    }
}
