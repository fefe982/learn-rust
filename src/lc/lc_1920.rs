// https://leetcode.com/problems/build-array-from-permutation/
// 1920. Build Array from Permutation
pub struct Solution;
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        for i in 0..nums.len() {
            ans[i] = nums[nums[i] as usize];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_array() {
        assert_eq!(Solution::build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
        assert_eq!(Solution::build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);
    }
}
