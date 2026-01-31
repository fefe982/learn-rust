// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i/
// 3010. Divide an Array into Subarrays With Minimum Cost I
pub struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut least = i32::MAX;
        let mut least1 = i32::MAX;
        for i in 1..nums.len() {
            if nums[i] < least1 {
                least1 = nums[i];
            }
            if nums[i] < least {
                least1 = least;
                least = nums[i];
            }
        }
        nums[0] + least + least1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divide_array() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
        assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
        assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
    }
}
