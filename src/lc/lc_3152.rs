// https://leetcode.com/problems/special-array-ii/
// 3152. Special Array II
pub struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut nums = nums;
        nums[0] = nums[0] % 2;
        for i in 1..nums.len() {
            nums[i] = (nums[i - 1] + nums[i]) % 2 + nums[i - 1];
        }
        queries
            .into_iter()
            .map(|q| nums[q[1] as usize] - nums[q[0] as usize] == q[1] - q[0])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_array_special() {
        assert_eq!(
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec_vec![[0, 4]]),
            [false]
        );
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec_vec![[0, 2], [2, 3]]),
            [false, true]
        );
    }
}
