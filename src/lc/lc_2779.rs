// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/
// 2779. Maximum Beauty of an Array After Applying Operation
pub struct Solution;
impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let mut j = nums.partition_point(|&x| x <= nums[0] + k * 2);
        let mut ans = j - i;
        while j < nums.len() {
            while nums[j] - nums[i] > k * 2 {
                i += 1;
            }
            while j < nums.len() && nums[j] - nums[i] <= k * 2 {
                j += 1;
            }
            ans = ans.max(j - i);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_beauty() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
        assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
