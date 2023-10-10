// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/
// 2009. Minimum Number of Operations to Make Array Continuous
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len() as i32;
        let mut last = -1;
        nums.retain(|&x| {
            if x != last {
                last = x;
                true
            } else {
                false
            }
        });
        let mut pos0 = 0;
        let mut pos1 = nums.partition_point(|&x| x < nums[pos0] + len);
        let mut max = pos1 - pos0;
        while pos1 < nums.len() {
            pos0 += 1;
            while pos1 < nums.len() && nums[pos1] - nums[pos0] < len {
                pos1 += 1;
            }
            max = max.max(pos1 - pos0);
        }
        len - max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
        assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
    }
}
