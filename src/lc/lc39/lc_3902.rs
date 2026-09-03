// https://leetcode.com/problems/smallest-stable-index-i/
// 3902. Smallest Stable Group
pub struct Solution;
impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = nums.clone();
        for i in (0..min.len() - 1).rev() {
            min[i] = min[i].min(min[i + 1]);
        }
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            max = max.max(nums[i]);
            if max - min[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_stable_index() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 3), 3);
        assert_eq!(Solution::first_stable_index(vec![3, 2, 1], 1), -1);
    }
}
