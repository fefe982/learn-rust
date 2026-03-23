// https://leetcode.com/problems/equal-sum-arrays-with-minimum-number-of-operations/
// 1775. Equal Sum Arrays With Minimum Number of Operations
pub struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sum1 = nums1.iter().sum::<i32>();
        let sum2 = nums2.iter().sum::<i32>();
        if sum1 == sum2 {
            return 0;
        }
        let (mut low, mut high, mut diff) = if sum1 > sum2 {
            (nums2, nums1, sum1 - sum2)
        } else {
            (nums1, nums2, sum2 - sum1)
        };
        low.sort_unstable();
        high.sort_unstable();
        let mut cnt = 0;
        let mut i = 0;
        let mut j = high.len();
        while diff > 0 {
            let dlow = if i < low.len() { 6 - low[i] } else { 0 };
            let dhigh = if j > 0 { high[j - 1] - 1 } else { 0 };
            if dlow == 0 && dhigh == 0 {
                return -1;
            }
            if dlow >= dhigh {
                diff -= dlow;
                i += 1;
                cnt += 1;
            } else {
                diff -= dhigh;
                j -= 1;
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(
            Solution::min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2]),
            3
        );
        assert_eq!(Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6]), -1);
        assert_eq!(Solution::min_operations(vec![6, 6], vec![1]), 3);
    }
}
