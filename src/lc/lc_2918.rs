// https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/
// 2918. Minimum Equal Sum of Two Arrays With Replaced Elements
pub struct Solution;
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1 = 0i64;
        let mut nz1 = 0;
        let mut sum2 = 0i64;
        let mut nz2 = 0;
        for n in nums1 {
            if n == 0 {
                nz1 += 1;
                sum1 += 1;
            } else {
                sum1 += n as i64;
            }
        }
        for n in nums2 {
            if n == 0 {
                nz2 += 1;
                sum2 += 1;
            } else {
                sum2 += n as i64;
            }
        }
        if (sum1 > sum2 && nz2 == 0) || (sum1 < sum2 && nz1 == 0) {
            return -1;
        } else {
            return sum1.max(sum2);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_sum() {
        assert_eq!(Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
        assert_eq!(Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]), -1);
    }
}
