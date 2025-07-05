// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
// 1863. Sum of All Subset XOR Totals
pub struct Solution;
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.into_iter().fold(0, |acc, x| acc | x) << (n - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subset_xor_sum() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
