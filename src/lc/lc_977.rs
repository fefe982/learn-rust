// https://leetcode.com/problems/squares-of-a-sorted-array/
// 977. Squares of a Sorted Array
pub struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut s = 0;
        let mut e = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if nums[s].abs() > nums[e].abs() {
                res[i] = nums[s] * nums[s];
                s += 1;
            } else {
                res[i] = nums[e] * nums[e];
                e -= 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_squares() {
        assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
        assert_eq!(Solution::sorted_squares(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
    }
}
