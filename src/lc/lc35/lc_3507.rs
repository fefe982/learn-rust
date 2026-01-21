// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/
// Minimum Pair Removal to Sort Array I
pub struct Solution;
impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        super::lc_3510::Solution::minimum_pair_removal(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_pair_removal() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
