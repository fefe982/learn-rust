// https://leetcode.com/problems/smallest-stable-index-ii/
// 3904. Smallest Stable Index II
pub struct Solution;
impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        super::lc_3903::Solution::first_stable_index(nums, k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_stable_index() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 3), 3);
        assert_eq!(Solution::first_stable_index(vec![3, 2, 1], 1), -1);
        assert_eq!(Solution::first_stable_index(vec![0], 0), 0);
    }
}
