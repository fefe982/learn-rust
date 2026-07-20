// https://leetcode.com/problems/apply-operations-to-make-all-array-elements-equal-to-zero/
// 2772. Apply Operations to Make All Array Elements Equal to Zero
pub struct Solution;
impl Solution {
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut d = 0;
        let mut nums = nums;
        for i in 0..nums.len() {
            if nums[i] < d {
                return false;
            }
            if i + k <= nums.len() {
                nums[i] -= d;
                d += nums[i];
            } else if nums[i] != d {
                return false;
            }
            if i + 1 >= k {
                d -= nums[i + 1 - k];
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_array() {
        assert_eq!(Solution::check_array(vec![2, 2, 3, 1, 1, 0], 3), true);
        assert_eq!(Solution::check_array(vec![1, 3, 1, 1], 2), false);
    }
}
