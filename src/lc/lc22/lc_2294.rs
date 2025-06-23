// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
// 2294. Partition Array Such That Maximum Difference Is K
pub struct Solution;
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut l = nums[0];
        let mut c = 1;
        for i in 1..nums.len() {
            if nums[i] - l > k {
                l = nums[i];
                c += 1;
            }
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_array() {
        assert_eq!(Solution::partition_array(vec![3, 6, 1, 2, 5], 2), 2);
        assert_eq!(Solution::partition_array(vec![1, 2, 3], 1), 2);
    }
}
