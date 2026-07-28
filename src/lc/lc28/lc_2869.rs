// https://leetcode.com/problems/minimum-operations-to-collect-elements/
// 2869. Minimum Operations to Collect Elements
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut v = vec![false; k as usize + 1];
        let mut i = nums.len();
        let mut c = 0;
        while i > 0 {
            i -= 1;
            if nums[i] <= k && !v[nums[i] as usize] {
                v[nums[i] as usize] = true;
                c += 1;
                if c == k {
                    break;
                }
            }
        }
        (nums.len() - i) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 3), 5);
        assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
