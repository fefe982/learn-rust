// https://leetcode.com/problems/find-target-indices-after-sorting-array/
// 2089. Find Target Indices After Sorting Array
pub struct Solution;
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = Vec::new();
        for (i, n) in nums.into_iter().enumerate() {
            if n == target {
                ans.push(i as i32);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_target_indices() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
