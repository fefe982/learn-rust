// https://leetcode.com/problems/find-k-th-smallest-pair-distance/
// 719. Find K-th Smallest Pair Distance
pub struct Solution;
impl Solution {
    fn count(nums: &Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        let mut i = 0;
        let mut j = 1;
        while i < nums.len() {
            while j < nums.len() && nums[j] - nums[i] <= target {
                j += 1;
            }
            count += j - i - 1;
            i += 1;
        }
        count as i32
    }
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut min = 0;
        let mut max = *nums.last().unwrap() - *nums.first().unwrap();
        if Self::count(&nums, min) >= k {
            return min;
        }
        if k >= nums.len() as i32 * (nums.len() as i32 - 1) / 2 {
            return max;
        }
        while min + 1 < max {
            let mid = (min + max) / 2;
            if Self::count(&nums, mid) < k {
                min = mid;
            } else {
                max = mid;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_distance_pair() {
        assert_eq!(
            Solution::smallest_distance_pair(vec![9, 10, 7, 10, 6, 1, 5, 4, 9, 8], 18),
            2
        );
        assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
    }
}
