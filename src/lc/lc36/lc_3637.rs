// https://leetcode.com/problems/trionic-array-i/
// 3637. Trionic Array I
pub struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums.len() == 3 {
            return false;
        }
        let mut i = 0;
        if nums[i] >= nums[i + 1] {
            return false;
        }
        i += 1;
        while i + 1 < nums.len() && nums[i] < nums[i + 1] {
            i += 1;
        }
        if i + 1 == nums.len() || nums[i] == nums[i + 1] {
            return false;
        }
        while i + 1 < nums.len() && nums[i] > nums[i + 1] {
            i += 1;
        }
        if i + 1 == nums.len() || nums[i] == nums[i + 1] {
            return false;
        }
        while i + 1 < nums.len() && nums[i] < nums[i + 1] {
            i += 1;
        }
        i + 1 == nums.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_trionic() {
        assert_eq!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]), true);
        assert_eq!(Solution::is_trionic(vec![2, 1, 3]), false);
    }
}
