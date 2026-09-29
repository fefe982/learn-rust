// https://leetcode.com/problems/transform-array-by-parity/
// 3467. Transform Array by Parity
pub struct Solution;
impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut e = 0;
        for &n in &nums {
            if n % 2 == 0 {
                e += 1;
            }
        }
        let mut nums = nums;
        for i in 0..e {
            nums[i] = 0;
        }
        for i in e..nums.len() {
            nums[i] = 1;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transform_array() {
        assert_eq!(Solution::transform_array(vec![4, 3, 2, 1]), vec![0, 0, 1, 1]);
        assert_eq!(Solution::transform_array(vec![1, 5, 1, 4, 2]), vec![0, 0, 1, 1, 1]);
    }
}
