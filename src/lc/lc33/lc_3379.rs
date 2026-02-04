// https://leetcode.com/problems/transformed-array/
// 3379. Transform the Array
pub struct Solution;
impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::with_capacity(nums.len());
        for i in 0..n {
            res.push(nums[(i as i32 + nums[i] % (n as i32) + n as i32) as usize % n])
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_transformed_array() {
        assert_eq!(Solution::construct_transformed_array(vec![3, -2, 1, 1]), [1, 1, 1, 3]);
        assert_eq!(Solution::construct_transformed_array(vec![-1, 4, -1]), [-1, -1, 4]);
    }
}
