// https://leetcode.com/problems/maximum-distance-between-a-pair-of-values/
// 1855. Maximum Distance Between a Pair of Values
pub struct Solution;
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0usize;
        let mut j = 0usize;
        let mut answer = 0i32;

        while i < nums1.len() && j < nums2.len() {
            if i > j {
                j = i;
            }

            if j == nums2.len() {
                break;
            }

            if nums1[i] <= nums2[j] {
                answer = answer.max((j - i) as i32);
                j += 1;
            } else {
                i += 1;
            }
        }

        answer
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_distance() {
        assert_eq!(
            Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
            2
        );
        assert_eq!(Solution::max_distance(vec![2, 2, 2], vec![10, 10, 1]), 1);
        assert_eq!(Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]), 2);
        assert_eq!(Solution::max_distance(vec![5, 4], vec![3, 2]), 0);
    }
}
