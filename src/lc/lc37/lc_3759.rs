// https://leetcode.com/problems/count-elements-with-at-least-k-greater-values/
// 3759. Count Elements With at Least K Greater Values
pub struct Solution;
impl Solution {
    pub fn count_elements(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return nums.len() as i32;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let k = k as usize;
        for i in (0..nums.len() - k).rev() {
            if nums[i] < nums[i + 1] {
                return i as i32 + 1;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_elements() {
        assert_eq!(Solution::count_elements(vec![3, 1, 2], 1), 2);
        assert_eq!(Solution::count_elements(vec![5, 5, 5], 2), 0);
    }
}
