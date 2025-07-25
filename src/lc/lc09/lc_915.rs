// https://leetcode.com/problems/partition-array-into-disjoint-intervals/
// 915. Partition Array into Disjoint Intervals
pub struct Solution;
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut min = vec![0; nums.len()];
        min[nums.len() - 1] = nums[nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            min[i] = nums[i].min(min[i + 1]);
        }
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            max = max.max(nums[i]);
            if max <= min[i + 1] {
                return i as i32 + 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_disjoint() {
        assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }
}
