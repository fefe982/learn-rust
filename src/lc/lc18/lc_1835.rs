// https://leetcode.com/problems/find-xor-sum-of-all-pairs-bitwise-and/
// 1835. Find XOR Sum of All Pairs Bitwise AND
pub struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        arr1.into_iter().fold(0, |acc, x| acc ^ x) & arr2.into_iter().fold(0, |acc, x| acc ^ x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_xor_sum() {
        assert_eq!(Solution::get_xor_sum(vec![1, 2, 3], vec![6, 5]), 0);
        assert_eq!(Solution::get_xor_sum(vec![12], vec![4]), 4);
    }
}
