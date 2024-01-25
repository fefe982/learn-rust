// https://leetcode.com/problems/sum-of-values-at-indices-with-k-set-bits/
// 2859. Sum of Values at Indices With K Set Bits
pub struct Solution;
impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if i.count_ones() == k as u32 {
                sum += n;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_indices_with_k_set_bits() {
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1), 13);
        assert_eq!(Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2), 1);
    }
}
