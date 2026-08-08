// https://leetcode.com/problems/maximum-strong-pair-xor-i/
// 2932. Maximum Strong Pair XOR
pub struct Solution;
impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        super::lc_2935::Solution::maximum_strong_pair_xor(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_strong_pair_xor() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
    }
}
