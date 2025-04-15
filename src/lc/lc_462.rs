// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/
// 462. Minimum Moves to Equal Array Elements II
pub struct Solution;
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let c = nums[nums.len() / 2];
        nums.iter().map(|x| (x - c).abs()).sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_moves2() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    }
}
