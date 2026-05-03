// https://leetcode.com/problems/concatenation-of-array/
// 1929. Concatenation of Array
pub struct Solution;
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        for i in 0..n {
            nums.push(nums[i]);
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_concatenation() {
        assert_eq!(Solution::get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        )
    }
}
